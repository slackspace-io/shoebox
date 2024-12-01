use std::fs;
use serde::{Deserialize, Serialize};
use leptos::prelude::Render;
use crate::db::insert_media_file;
use crate::lib_models::FileType;

pub async fn scan_files(dir: &str) -> Vec<FileType> {
    let mut files = Vec::new();

    // Iterate over entries in the specified directory
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif") {
                    insert_media_file("photo", path.display().to_string().as_str());
                    #[cfg(feature = "ssr")]
                    files.push(FileType::Photo(path.display().to_string()));
                } else if matches!(ext.as_str(), "mp4" | "mkv" | "avi" | "mov") {
                    insert_media_file("video", path.display().to_string().as_str());
                    #[cfg(feature = "ssr")]
                    files.push(FileType::Video(path.display().to_string()));
                } else {
                    files.push(FileType::Other(path.display().to_string()));
                }
            }
        }
    }

    files
}
