use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::error;

use crate::error::{AppError, Result};

/// Check if a file exists
pub async fn file_exists(path: &Path) -> bool {
    fs::metadata(path).await.is_ok()
}

/// Get file extension as lowercase string
pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
}

/// Check if a file is a video file
pub fn is_video_file(path: &Path) -> bool {
    if let Some(ext) = get_file_extension(path) {
        return ["mp4", "mov", "mkv"].contains(&ext.as_str());
    }
    false
}

/// Create directory if it doesn't exist
pub async fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path).await.map_err(|e| {
            AppError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create directory {}: {}", path.display(), e),
            ))
        })?;
    }
    Ok(())
}

/// Get a unique filename in a directory
pub async fn get_unique_filename(dir: &Path, base_name: &str, extension: &str) -> Result<PathBuf> {
    let mut counter = 0;
    let mut file_name = format!("{}.{}", base_name, extension);
    let mut file_path = dir.join(&file_name);

    while file_exists(&file_path).await {
        counter += 1;
        file_name = format!("{}_{}.{}", base_name, counter, extension);
        file_path = dir.join(&file_name);
    }

    Ok(file_path)
}

/// Copy a file with error handling
pub async fn copy_file(source: &Path, dest: &Path) -> Result<()> {
    if !source.exists() {
        return Err(AppError::NotFound(format!(
            "Source file not found: {}",
            source.display()
        )));
    }

    // Ensure parent directory exists
    if let Some(parent) = dest.parent() {
        ensure_dir_exists(parent).await?;
    }

    fs::copy(source, dest).await.map_err(|e| {
        error!(
            "Failed to copy {} to {}: {}",
            source.display(),
            dest.display(),
            e
        );
        AppError::Io(e)
    })?;

    Ok(())
}
