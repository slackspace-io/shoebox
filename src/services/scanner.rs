use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::fs;
use walkdir::WalkDir;
use tracing::{info, warn, error};
use anyhow::Result;

use crate::error::AppError;
use crate::models::{Video, CreateVideoDto};
use crate::services::video::VideoService;
use crate::services::thumbnail::ThumbnailService;

pub struct ScannerService;

impl ScannerService {
    pub async fn scan_directories(
        paths: &[String],
        video_service: &VideoService,
        thumbnail_service: &ThumbnailService,
    ) -> Result<Vec<Video>, AppError> {
        let mut new_videos = Vec::new();

        for path in paths {
            info!("Scanning directory: {}", path);
            let path = Path::new(path);

            if !path.exists() {
                warn!("Path does not exist: {}", path.display());
                continue;
            }

            let entries = match Self::get_video_files(path) {
                Ok(entries) => entries,
                Err(e) => {
                    error!("Error scanning directory {}: {}", path.display(), e);
                    continue;
                }
            };

            for entry in entries {
                let file_path = entry.path().to_string_lossy().to_string();
                let file_name = entry.file_name().to_string_lossy().to_string();

                // Check if video already exists in database
                if video_service.find_by_path(&file_path).await.is_ok() {
                    continue;
                }

                info!("Found new video: {}", file_path);

                // Get file metadata
                let metadata = match fs::metadata(&file_path).await {
                    Ok(m) => m,
                    Err(e) => {
                        error!("Error getting metadata for {}: {}", file_path, e);
                        continue;
                    }
                };

                // Get created date
                let created_date = match metadata.created() {
                    Ok(time) => {
                        let datetime: chrono::DateTime<chrono::Utc> = time.into();
                        Some(datetime.to_rfc3339())
                    },
                    Err(_) => None,
                };

                // Generate thumbnail
                let thumbnail_path = match thumbnail_service.generate_thumbnail(&file_path).await {
                    Ok(path) => Some(path),
                    Err(e) => {
                        error!("Error generating thumbnail for {}: {}", file_path, e);
                        None
                    }
                };

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
                    tags: Vec::new(),
                    people: Vec::new(),
                };

                match video_service.create(create_dto).await {
                    Ok(video) => {
                        new_videos.push(video);
                    },
                    Err(e) => {
                        error!("Error creating video record: {}", e);
                    }
                }
            }
        }

        info!("Scan complete. Found {} new videos", new_videos.len());
        Ok(new_videos)
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
