// filesystem/mod.rs
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum FileType {
    Video(String),
    Photo(String),
    Other(String),
}

pub fn scan_files(dir: &str) -> Vec<FileType> {
    let mut files = Vec::new();

    // Iterate over entries in the specified directory
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif") {
                    files.push(FileType::Photo(path.display().to_string()));
                } else if matches!(ext.as_str(), "mp4" | "mkv" | "avi" | "mov") {
                    files.push(FileType::Video(path.display().to_string()));
                } else {
                    files.push(FileType::Other(path.display().to_string()));
                }
            }
        }
    }

    files
}