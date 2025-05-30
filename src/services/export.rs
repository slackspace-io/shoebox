use std::path::{Path, PathBuf};
use tokio::fs;
use chrono::Utc;
use tracing::{info, error};
use serde_json::json;

use crate::error::{AppError, Result};
use crate::config::Config;
use crate::models::{ExportRequest, VideoWithMetadata};
use crate::services::video::VideoService;

pub struct ExportService {
    config: Config,
    video_service: VideoService,
    export_base_path: PathBuf,
}

impl ExportService {
    pub fn new(config: Config, video_service: VideoService) -> Self {
        let export_base_path = PathBuf::from(&config.media.export_base_path);
        Self {
            config,
            video_service,
            export_base_path,
        }
    }

    pub async fn export_videos(&self, request: ExportRequest) -> Result<String> {
        // Create export directory with timestamp and project name
        let date = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let project_dir_name = format!("{}_{}", date, request.project_name.replace(" ", "_"));
        let project_dir = self.export_base_path.join(&project_dir_name);

        // Ensure export base directory exists
        if !self.export_base_path.exists() {
            fs::create_dir_all(&self.export_base_path).await.map_err(|e| {
                AppError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create export directory: {}", e),
                ))
            })?;
        }

        // Create project directory
        fs::create_dir_all(&project_dir).await.map_err(|e| {
            AppError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create project directory: {}", e),
            ))
        })?;

        info!("Exporting videos to {}", project_dir.display());

        // Collect videos with metadata
        let mut videos_with_metadata = Vec::new();
        for video_id in &request.video_ids {
            match self.video_service.find_with_metadata(video_id).await {
                Ok(video_metadata) => {
                    videos_with_metadata.push(video_metadata);
                }
                Err(e) => {
                    error!("Error fetching video {}: {}", video_id, e);
                    return Err(e);
                }
            }
        }

        // Copy videos to export directory
        for video_metadata in &videos_with_metadata {
            // Determine source path based on configuration
            let source_path = if request.use_original_files && video_metadata.video.original_file_path.is_some() {
                Path::new(video_metadata.video.original_file_path.as_ref().unwrap())
            } else {
                Path::new(&video_metadata.video.file_path)
            };

            // Determine destination file name
            let dest_file_name = if request.use_original_files && video_metadata.video.original_file_path.is_some() {
                // Extract the file name from the original file path
                Path::new(video_metadata.video.original_file_path.as_ref().unwrap())
                    .file_name()
                    .unwrap_or_else(|| std::ffi::OsStr::new(&video_metadata.video.file_name))
                    .to_string_lossy()
                    .to_string()
            } else {
                video_metadata.video.file_name.clone()
            };

            let dest_path = project_dir.join(&dest_file_name);

            // Copy the file
            match fs::copy(source_path, &dest_path).await {
                Ok(_) => {
                    info!(
                        "Copied {} to {}",
                        source_path.display(),
                        dest_path.display()
                    );
                }
                Err(e) => {
                    error!(
                        "Failed to copy {} to {}: {}",
                        source_path.display(),
                        dest_path.display(),
                        e
                    );
                    return Err(AppError::Io(e));
                }
            }
        }

        // Create metadata.json
        let metadata = json!({
            "project_name": request.project_name,
            "export_date": Utc::now().to_rfc3339(),
            "videos": videos_with_metadata.iter().map(|v| {
                json!({
                    "id": v.video.id,
                    "file_name": v.video.file_name,
                    "title": v.video.title,
                    "description": v.video.description,
                    "created_date": v.video.created_date,
                    "file_size": v.video.file_size,
                    "rating": v.video.rating,
                    "tags": v.tags,
                    "people": v.people,
                })
            }).collect::<Vec<_>>(),
        });

        let metadata_path = project_dir.join("metadata.json");
        let metadata_json = serde_json::to_string_pretty(&metadata).map_err(|e| {
            AppError::InternalServerError(format!("Failed to serialize metadata: {}", e))
        })?;

        fs::write(&metadata_path, metadata_json).await.map_err(|e| {
            AppError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to write metadata file: {}", e),
            ))
        })?;

        info!("Created metadata file at {}", metadata_path.display());

        // Return the path to the export directory
        Ok(project_dir.to_string_lossy().to_string())
    }
}
