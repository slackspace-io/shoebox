use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::fs;
use uuid::Uuid;
use tracing::{info, error};

use crate::error::{AppError, Result};
use crate::config::Config;

pub struct ThumbnailService {
    thumbnail_dir: PathBuf,
}

impl ThumbnailService {
    pub fn new(config: &Config) -> Self {
        let thumbnail_dir = PathBuf::from(&config.media.thumbnail_path);
        Self { thumbnail_dir }
    }

    pub async fn generate_thumbnail(&self, video_path: &str) -> Result<String> {
        // Ensure thumbnail directory exists
        if !self.thumbnail_dir.exists() {
            fs::create_dir_all(&self.thumbnail_dir).await.map_err(|e| {
                AppError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create thumbnail directory: {}", e),
                ))
            })?;
        }

        // Generate a unique filename for the thumbnail
        let thumbnail_filename = format!("{}.jpg", Uuid::new_v4());
        let thumbnail_path = self.thumbnail_dir.join(&thumbnail_filename);
        let thumbnail_path_str = thumbnail_path.to_string_lossy().to_string();

        info!("Generating thumbnail for {} at {}", video_path, thumbnail_path_str);

        // Use FFmpeg to extract the first keyframe
        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(video_path)
            .arg("-vf")
            .arg("select=eq(n\\,0)")
            .arg("-vframes")
            .arg("1")
            .arg("-y") // Overwrite output file if it exists
            .arg(&thumbnail_path_str)
            .output()
            .map_err(|e| {
                error!("FFmpeg command failed: {}", e);
                AppError::FFmpeg(format!("Failed to execute FFmpeg: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg error: {}", stderr);
            return Err(AppError::FFmpeg(format!("FFmpeg error: {}", stderr)));
        }

        // Check if thumbnail was created
        if !thumbnail_path.exists() {
            return Err(AppError::FFmpeg("Thumbnail was not created".to_string()));
        }

        Ok(thumbnail_path_str)
    }

    pub async fn delete_thumbnail(&self, thumbnail_path: &str) -> Result<()> {
        let path = Path::new(thumbnail_path);

        // Only delete if the file is in our thumbnail directory
        if path.starts_with(&self.thumbnail_dir) && path.exists() {
            fs::remove_file(path).await.map_err(AppError::Io)?;
            info!("Deleted thumbnail: {}", thumbnail_path);
        }

        Ok(())
    }
}
