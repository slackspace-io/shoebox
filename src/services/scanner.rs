use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::fs;
use walkdir::WalkDir;
use tracing::{info, warn, error};
use anyhow::Result;
use std::io::{Read, Seek, SeekFrom};
use std::process::Command;
use tokio::task;
use tokio::process::Command as TokioCommand;
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::error::AppError;
use crate::models::{Video, CreateVideoDto};
use crate::services::video::VideoService;
use crate::services::thumbnail::ThumbnailService;

pub struct ScannerService;

impl ScannerService {
    // Extract creation date from video file using FFprobe (part of FFmpeg suite)
    async fn get_video_creation_date(path: &str) -> Option<String> {
        info!("Attempting to extract creation date from video metadata for: {}", path);

        // Try multiple metadata tags that might contain creation date information
        // Different video formats store this information in different tags
        let possible_tags = [
            "creation_time",           // Common in MP4
            "com.apple.quicktime.creationdate", // Common in MOV
            "date",                    // Generic date tag
            "com.apple.quicktime.createdate", // Alternative MOV tag
        ];

        for tag in possible_tags.iter() {
            // Use ffprobe instead of ffmpeg for metadata extraction
            let output = match TokioCommand::new("ffprobe")
                .arg("-v")
                .arg("error")
                .arg("-show_entries")
                .arg(format!("format_tags={}", tag))
                .arg("-of")
                .arg("default=noprint_wrappers=1:nokey=1")
                .arg(path)
                .output()
                .await {
                    Ok(output) => output,
                    Err(e) => {
                        error!("Failed to execute FFprobe for metadata extraction (tag: {}): {}", tag, e);
                        continue; // Try next tag
                    }
                };

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.is_empty() {
                    error!("FFprobe metadata extraction error (tag: {}): {}", tag, stderr);
                }
                continue; // Try next tag
            }

            let stdout = String::from_utf8_lossy(&output.stdout);
            let creation_time = stdout.trim();

            if creation_time.is_empty() {
                info!("No {} found in video metadata", tag);
                continue; // Try next tag
            }

            info!("Found {} tag with value: {}", tag, creation_time);

            // Try multiple date formats
            let date_formats = [
                "%Y-%m-%dT%H:%M:%S%.fZ",  // ISO 8601 with fractional seconds
                "%Y-%m-%d %H:%M:%S",      // Simple date time format
                "%Y:%m:%d %H:%M:%S",      // EXIF date format (common in photos/videos)
                "%Y-%m-%dT%H:%M:%S%z",    // ISO 8601 with timezone
                "%Y-%m-%d",               // Just date
            ];

            for format in date_formats.iter() {
                match chrono::NaiveDateTime::parse_from_str(creation_time, format) {
                    Ok(dt) => {
                        let datetime = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc);
                        info!("Extracted creation date from video metadata (tag: {}, format: {}): {}",
                              tag, format, datetime.to_rfc3339());
                        return Some(datetime.to_rfc3339());
                    },
                    Err(_) => {
                        // Try next format
                    }
                }

                // Also try parsing as DateTime which handles timezone information
                match chrono::DateTime::parse_from_str(creation_time, format) {
                    Ok(dt) => {
                        let utc_time = dt.with_timezone(&chrono::Utc);
                        info!("Extracted creation date from video metadata (tag: {}, format: {}): {}",
                              tag, format, utc_time.to_rfc3339());
                        return Some(utc_time.to_rfc3339());
                    },
                    Err(_) => {
                        // Try next format
                    }
                }
            }

            // If we got here, we found a tag but couldn't parse the date
            error!("Found {} tag but couldn't parse date value: {}", tag, creation_time);
        }

        // If we got here, we couldn't find any usable creation date
        info!("No usable creation date found in video metadata");
        None
    }

    // Check if a file extension is a supported video format
    fn is_supported_video_format(ext: &str) -> bool {
        // List of supported video formats
        ["mp4", "mov", "mkv", "avi", "wmv", "flv", "webm"].contains(&ext)
    }

    // Get video duration using FFprobe
    async fn get_video_duration(path: &str) -> Option<i64> {
        info!("Attempting to extract duration from video: {}", path);

        let output = match TokioCommand::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-show_entries")
            .arg("format=duration")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg(path)
            .output()
            .await {
                Ok(output) => output,
                Err(e) => {
                    error!("Failed to execute FFprobe for duration extraction: {}", e);
                    return None;
                }
            };

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.is_empty() {
                error!("FFprobe duration extraction error: {}", stderr);
            }
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let duration_str = stdout.trim();

        if duration_str.is_empty() {
            info!("No duration found in video metadata");
            return None;
        }

        // Parse the duration string to a float and convert to milliseconds
        match duration_str.parse::<f64>() {
            Ok(duration_seconds) => {
                let duration_ms = (duration_seconds * 1000.0) as i64;
                info!("Extracted duration from video: {} seconds ({} ms)", duration_seconds, duration_ms);
                Some(duration_ms)
            },
            Err(e) => {
                error!("Failed to parse duration '{}': {}", duration_str, e);
                None
            }
        }
    }

    // Function to get the duration of an MP4 file (if possible)
    // This is a fallback method for MP4 files when ffprobe fails
    fn get_mp4_duration(path: &PathBuf) -> Option<f64> {
        // Try to open the file in blocking mode for quick analysis
        let file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(_) => return None, // If we can't open the file, return None
        };

        // Read the file to find the moov atom and extract duration
        let mut reader = std::io::BufReader::new(file);
        let mut buffer = [0u8; 8]; // 8 bytes for atom size (4) and type (4)
        let mut position = 0;

        // We'll search the entire file for the moov atom
        loop {
            // Read atom header
            match reader.read_exact(&mut buffer) {
                Ok(_) => {},
                Err(_) => break, // End of file or error
            }

            // Parse atom size (big-endian)
            let size = ((buffer[0] as u32) << 2) |
                      ((buffer[1] as u32) << 16) |
                      ((buffer[2] as u32) << 8) |
                      (buffer[3] as u32);

            // Check if this is the moov atom
            if &buffer[4..8] == b"moov" {
                // Found moov atom, now look for mvhd atom inside it
                let mut mvhd_buffer = vec![0u8; size as usize - 8];
                if reader.read_exact(&mut mvhd_buffer).is_err() {
                    break;
                }

                // Search for mvhd atom inside moov
                let mut i = 0;
                while i + 8 <= mvhd_buffer.len() {
                    let atom_size = ((mvhd_buffer[i] as u32) << 24) |
                                   ((mvhd_buffer[i+1] as u32) << 16) |
                                   ((mvhd_buffer[i+2] as u32) << 8) |
                                   (mvhd_buffer[i+3] as u32);

                    if &mvhd_buffer[i+4..i+8] == b"mvhd" {
                        // Found mvhd atom, extract duration
                        // The format depends on the version (first byte after atom header)
                        let version = mvhd_buffer[i+8];

                        if version == 0 && i + 20 + 12 < mvhd_buffer.len() {
                            // Version 0: 32-bit duration at offset 20
                            let time_scale = ((mvhd_buffer[i+16] as u32) << 24) |
                                            ((mvhd_buffer[i+17] as u32) << 16) |
                                            ((mvhd_buffer[i+18] as u32) << 8) |
                                            (mvhd_buffer[i+19] as u32);

                            let duration = ((mvhd_buffer[i+20] as u32) << 24) |
                                          ((mvhd_buffer[i+21] as u32) << 16) |
                                          ((mvhd_buffer[i+22] as u32) << 8) |
                                          (mvhd_buffer[i+23] as u32);

                            if time_scale > 0 {
                                return Some(duration as f64 / time_scale as f64);
                            }
                        } else if version == 1 && i + 28 + 12 < mvhd_buffer.len() {
                            // Version 1: 64-bit duration at offset 28
                            let time_scale = ((mvhd_buffer[i+24] as u32) << 24) |
                                            ((mvhd_buffer[i+25] as u32) << 16) |
                                            ((mvhd_buffer[i+26] as u32) << 8) |
                                            (mvhd_buffer[i+27] as u32);

                            let duration = ((mvhd_buffer[i+28] as u64) << 56) |
                                          ((mvhd_buffer[i+29] as u64) << 48) |
                                          ((mvhd_buffer[i+30] as u64) << 40) |
                                          ((mvhd_buffer[i+31] as u64) << 32) |
                                          ((mvhd_buffer[i+32] as u64) << 24) |
                                          ((mvhd_buffer[i+33] as u64) << 16) |
                                          ((mvhd_buffer[i+34] as u64) << 8) |
                                          (mvhd_buffer[i+35] as u64);

                            if time_scale > 0 {
                                return Some(duration as f64 / time_scale as f64);
                            }
                        }

                        break;
                    }

                    // Move to the next atom
                    if atom_size > 0 {
                        i += atom_size as usize;
                    } else {
                        break;
                    }
                }

                break;
            }

            // Skip to the next atom
            if size > 8 {
                // Skip the rest of this atom (size - 8 bytes we already read)
                let to_skip = size as u64 - 8;
                match reader.seek(SeekFrom::Current(to_skip as i64)) {
                    Ok(new_pos) => position = new_pos,
                    Err(_) => break, // Error seeking
                }
            } else if size == 0 {
                // Size 0 means the rest of the file, so we're done
                break;
            } else if size < 8 {
                // Invalid size, something is wrong
                break;
            }
        }

        // If we get here, we couldn't find the duration
        None
    }

    pub async fn scan_directories(
        path_configs: &[crate::config::MediaPathConfig],
        video_service: VideoService,
        thumbnail_service: ThumbnailService,
        config: &crate::config::Config,
    ) -> Result<(Arc<tokio::sync::Mutex<Vec<Video>>>, Arc<tokio::sync::Mutex<Vec<Video>>>, Vec<tokio::task::JoinHandle<()>>), AppError> {
        // Wrap services in Arc for sharing across tasks
        let video_service = Arc::new(video_service);
        let thumbnail_service = Arc::new(thumbnail_service);

        // Create a semaphore to limit concurrent tasks
        // This prevents resource exhaustion when scanning large directories
        let max_concurrent_tasks = config.media.max_concurrent_tasks;
        info!("Limiting concurrent processing tasks to {}", max_concurrent_tasks);
        let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));

        // Create a single map to store all original file paths
        // This allows us to find original files regardless of which subdirectory they're in
        let all_original_files_arc = Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new()));

        // Pre-scan all original directories to build the map (in parallel)
        let mut prescan_tasks = Vec::new();

        for path_config in path_configs {
            if let Some(original_path) = &path_config.original_path {
                info!("Pre-scanning original directory: {}", original_path);
                let original_path_str = original_path.clone();
                let original_extension = path_config.original_extension.clone();
                let all_original_files = all_original_files_arc.clone();

                // Spawn a task for each original directory
                let prescan_task = task::spawn(async move {
                    let original_path_obj = Path::new(&original_path_str);
                    if !original_path_obj.exists() {
                        warn!("Original path does not exist: {}", original_path_str);
                        return;
                    }

                    let mut files_count = 0;
                    let mut local_files = std::collections::HashMap::new();

                    // Walk through all files in the original directory and its subdirectories
                    for entry in WalkDir::new(original_path_obj).follow_links(true).into_iter().filter_map(|e| e.ok()) {
                        let path = entry.path();

                        if path.is_file() {
                            // Check if we should filter by extension
                            let should_include = if let Some(ref original_extension) = original_extension {
                                if let Some(ext) = path.extension() {
                                    ext.to_string_lossy().to_lowercase() == original_extension.to_lowercase()
                                } else {
                                    false
                                }
                            } else {
                                // If no extension specified, include all files
                                true
                            };

                            if should_include {
                                if let Some(file_stem) = path.file_stem() {
                                    let file_stem_str = file_stem.to_string_lossy().to_string();
                                    let full_path = path.to_string_lossy().to_string();

                                    // Store the full path with the file stem as the key
                                    local_files.insert(file_stem_str, full_path);
                                    files_count += 1;
                                }
                            }
                        }
                    }

                    // Update the shared map with our findings
                    let mut all_files_guard = all_original_files.lock().await;
                    for (key, value) in local_files {
                        all_files_guard.insert(key, value);
                    }

                    info!("Found {} original files in {} and its subdirectories", files_count, original_path_str);
                });

                prescan_tasks.push(prescan_task);
            }
        }

        // Wait for all pre-scan tasks to complete
        for task in prescan_tasks {
            let _ = task.await;
        }

        // Create vectors to hold all the tasks and results
        let mut tasks = Vec::new();
        let new_videos_arc = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let updated_videos_arc = Arc::new(tokio::sync::Mutex::new(Vec::new()));

        // Collect all video files from all directories first
        let mut all_entries = Vec::new();

        for path_config in path_configs {
            info!("Scanning directory: {}", path_config.path);
            let path = Path::new(&path_config.path);

            if !path.exists() {
                warn!("Path does not exist: {}", path.display());
                continue;
            }

            match Self::get_video_files(path) {
                Ok(entries) => {
                    for entry in entries {
                        all_entries.push((entry, path_config.clone()));
                    }
                },
                Err(e) => {
                    error!("Error scanning directory {}: {}", path.display(), e);
                }
            }
        }

        info!("Found {} video files to process", all_entries.len());

        // Process files in parallel with semaphore to limit concurrency
        for (entry, path_config) in all_entries {
            let file_path = entry.path().to_string_lossy().to_string();
            let file_name = entry.file_name().to_string_lossy().to_string();

            // Clone Arc pointers for the async task
            let video_service = video_service.clone();
            let thumbnail_service = thumbnail_service.clone();
            let all_original_files = all_original_files_arc.clone();
            let new_videos = new_videos_arc.clone();
            let updated_videos = updated_videos_arc.clone();
            let semaphore = semaphore.clone();

            // Spawn a task to process this file
            let task = task::spawn(async move {
                // Acquire a permit from the semaphore before processing
                // This will block if we've reached the maximum number of concurrent tasks
                let _permit = semaphore.acquire().await.unwrap();

                // Get file metadata
                let metadata = match fs::metadata(&file_path).await {
                    Ok(m) => m,
                    Err(e) => {
                        error!("Error getting metadata for {}: {}", file_path, e);
                        return;
                    }
                };

                // Get created date - only use video metadata as filesystem metadata is not accurate
                info!("Extracting creation date from video metadata");
                let created_date = Self::get_video_creation_date(&file_path).await;

                // Generate thumbnail
                let thumbnail_path = match thumbnail_service.generate_thumbnail(&file_path).await {
                    Ok(path) => Some(path),
                    Err(e) => {
                        error!("Error generating thumbnail for {}: {}", file_path, e);
                        None
                    }
                };

                // Extract duration for video files
                let duration = if let Some(ext) = std::path::Path::new(&file_path).extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    if Self::is_supported_video_format(&ext) {
                        // Use ffprobe to get duration for all supported video formats
                        let ffprobe_duration = Self::get_video_duration(&file_path).await;

                        // Fallback to MP4 specific method for MP4 files if ffprobe failed
                        if ffprobe_duration.is_none() && ext == "mp4" {
                            Self::get_mp4_duration(&std::path::PathBuf::from(&file_path))
                                .map(|d| d as i64)
                        } else {
                            ffprobe_duration
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Check for original file if original_path is specified
                let original_file_path = if let Some(original_path) = &path_config.original_path {
                    // Get the file name without extension
                    let file_stem = std::path::Path::new(&file_name)
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string());

                    if let Some(stem) = file_stem {
                        // Look up the file stem in our pre-built map
                        let all_files_guard = all_original_files.lock().await;
                        if let Some(original_file) = all_files_guard.get(&stem) {
                            info!("Found original file: {}", original_file);
                            Some(original_file.clone())
                        } else {
                            // If not found in the map, we couldn't find the original file
                            let extension_info = if let Some(ext) = &path_config.original_extension {
                                format!(" with extension '{}'", ext)
                            } else {
                                "".to_string()
                            };
                            info!("Original file not found for stem: '{}'{} in path: '{}'",
                                  stem, extension_info, original_path);
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Check if video already exists in database
                match video_service.find_by_path(&file_path).await {
                    Ok(existing_video) => {
                        // Update existing video with new metadata
                        info!("Updating existing video: {}", file_path);
                        match video_service.update_technical_metadata(
                            &existing_video.id,
                            Some(metadata.len() as i64),
                            duration,
                            created_date,
                            thumbnail_path,
                            original_file_path
                        ).await {
                            Ok(updated_video) => {
                                let mut updated_videos_guard = updated_videos.lock().await;
                                updated_videos_guard.push(updated_video);
                            },
                            Err(e) => {
                                error!("Error updating video metadata: {}", e);
                            }
                        }
                        return;
                    },
                    Err(_) => {
                        // Video doesn't exist, continue with creation
                    }
                };

                info!("Found new video: {}", file_path);

                // Create video record
                let file_name_clone = file_name.clone();

                let create_dto = CreateVideoDto {
                    file_path,
                    file_name,
                    title: Some(file_name_clone),
                    description: None,
                    created_date,
                    file_size: Some(metadata.len() as i64),
                    thumbnail_path,
                    rating: None,
                    duration,
                    tags: Vec::new(),
                    people: Vec::new(),
                    original_file_path,
                };

                match video_service.create(create_dto).await {
                    Ok(video) => {
                        let mut new_videos_guard = new_videos.lock().await;
                        new_videos_guard.push(video);
                    },
                    Err(e) => {
                        error!("Error creating video record: {}", e);
                    }
                }

                // The permit is automatically released when it goes out of scope
            });

            tasks.push(task);
        }

        // Return the shared results and tasks without waiting
        info!("Scan started with {} tasks", tasks.len());
        Ok((new_videos_arc, updated_videos_arc, tasks))
    }

    // Helper method to collect results from a scan
    pub async fn collect_scan_results(
        new_videos_arc: Arc<tokio::sync::Mutex<Vec<Video>>>,
        updated_videos_arc: Arc<tokio::sync::Mutex<Vec<Video>>>,
        tasks: Vec<tokio::task::JoinHandle<()>>
    ) -> Result<(Vec<Video>, Vec<Video>), AppError> {
        // Wait for all tasks to complete
        for task in tasks {
            let _ = task.await;
        }

        // Collect results
        let new_videos = {
            let guard = new_videos_arc.lock().await;
            guard.clone()
        };

        let updated_videos = {
            let guard = updated_videos_arc.lock().await;
            guard.clone()
        };

        info!("Scan complete. Found {} new videos and updated {} existing videos", new_videos.len(), updated_videos.len());
        Ok((new_videos, updated_videos))
    }

    fn get_video_files(dir: &Path) -> Result<Vec<walkdir::DirEntry>> {
        let mut video_files = Vec::new();

        for entry in WalkDir::new(dir).follow_links(true).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    if ["mp4", "mov", "mkv"].contains(&ext.as_str()) {
                        video_files.push(entry);
                    }
                }
            }
        }

        Ok(video_files)
    }

    pub fn is_video_file(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            return ["mp4", "mov", "mkv"].contains(&ext.as_str());
        }
        false
    }
}

// Utility function for Path to String conversion
pub fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().to_string()
}
