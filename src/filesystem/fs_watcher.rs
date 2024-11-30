use std::fs;

#[derive(Debug)]
pub enum FileType {
    Video(String),
    Photo(String),
    Other(String),
}


pub fn scan_files(dir: &str) -> Vec<FileType> {
    #[cfg(feature = "ssr")]
    use crate::db::db_calls::insert_media_file;
    let mut files = Vec::new();

    // Iterate over entries in the specified directory
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif") {
                    #[cfg(feature = "ssr")]
                    insert_media_file(path.file_name().unwrap().to_str().unwrap(), path.display().to_string().as_str());
                    files.push(FileType::Photo(path.display().to_string()));
                } else if matches!(ext.as_str(), "mp4" | "mkv" | "avi" | "mov") {
                    #[cfg(feature = "ssr")]
                    insert_media_file(path.file_name().unwrap().to_str().unwrap(), path.display().to_string().as_str());
                    files.push(FileType::Video(path.display().to_string()));
                } else {
                    files.push(FileType::Other(path.display().to_string()));
                }
            }
        }
    }

    files
}
