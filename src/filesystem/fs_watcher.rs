use crate::database::pg_calls::insert_new_media;
use crate::lib_models::{FileType, Metadata};
use crate::models::NewMedia;
use chrono::DateTime;
use leptos::logging::log;
use std::fs;

pub async fn scan_files(dir: &str) -> Vec<FileType> {
    let mut files = Vec::new();
    log!("initiating scan_files");
    println!("initiating scan_files");
    // Iterate over entries in the specified directory
    if let Ok(entries) = fs::read_dir(dir) {
        log!("Inside read_dir");
        for entry in entries.filter_map(Result::ok) {
            log!("Entry: {:?}", entry);
            let path = entry.path();
            let mut media_asset = Metadata {
                good_take: "not processed".to_string(),
                yearly_highlight: "not processed".to_string(),
                people: "not processed".to_string(),
                pets: "not processed".to_string(),
                location: "not processed".to_string(),
                processed: "not processed".to_string(),
                asset_type: "not processed".to_string(),
                path: path.display().to_string(),
                file_name: path.file_name().unwrap().to_string_lossy().to_string(),
                //todays date for creation
                creation_date: chrono::Local::now().to_string(),
                //todays date for discovery
                discovery_date: chrono::Local::now().to_string(),
            };
            let mut media_new = NewMedia {
                file_path: media_asset.path.clone(),
                file_name: media_asset.file_name.clone(),
                media_type: media_asset.asset_type.clone(),
                good_take: Option::from(true),
                highlight: Option::from(false),
                reviewed: Option::from(false),
                created_at: DateTime::from(chrono::Local::now()),
            };
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif") {
                    media_asset.asset_type = "photo".to_string();
                    media_new.media_type = "photo".to_string();
                    #[cfg(feature = "ssr")]
                    files.push(FileType::Photo(path.display().to_string()));
                } else if matches!(ext.as_str(), "mp4" | "mkv" | "avi" | "mov") {
                    media_asset.asset_type = "video".to_string();
                    media_new.media_type = "video".to_string();
                    #[cfg(feature = "ssr")]
                    files.push(FileType::Video(path.display().to_string()));
                } else {
                    media_asset.asset_type = "other".to_string();
                    media_new.media_type = "other".to_string();
                    files.push(FileType::Other(path.display().to_string()));
                }
                if media_asset.asset_type != "other" {
                    log!("Inserting media asset: {:?}", media_asset);
                    log!("Insert new media: {:?}", media_new);
                    insert_new_media(&media_new);
                }
            }
        }
    }

    files
}
