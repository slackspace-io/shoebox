use crate::database::pg_calls::{insert_new_media, update_media_original_path};
use crate::filesystem::exif_parse::parse_exif;
use crate::lib_models::{FileType, Metadata};
use crate::models::NewMedia;
use crate::settings::settings;
use anyhow::Result;
use chrono::DateTime;
use leptos::logging::log;
use std::fs;

pub async fn scan_all() {
    let settings = settings();
    for path in &settings.paths {
        let _ = scan_files(
            path.root_path.as_str(),
            path.route(&path.root_path).as_str(),
            path.root_path.as_str(),
        )
        .await;
        let dirs = find_dirs(path.root_path.as_str());
        for dir in dirs {
            let _ = scan_files(
                dir.as_str(),
                path.route(&path.root_path).as_str(),
                path.root_path.as_str(),
            )
            .await;
        }
    }
}

pub fn find_dirs(dir: &str) -> Vec<String> {
    let mut dirs = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            if entry.file_type().unwrap().is_dir() {
                dirs.push(entry.path().display().to_string());
                dirs.append(&mut find_dirs(entry.path().to_str().unwrap()));
            }
        }
    }
    dirs
}

pub async fn scan_files(dir: &str, route: &str, root_path: &str) -> Vec<FileType> {
    let mut files = Vec::new();
    log!("initiating scan_files");
    println!("initiating scan_files");
    let mut count = 0;
    // Iterate over entries in the specified directory
    if let Ok(entries) = fs::read_dir(dir) {
        //log!("Inside read_dir");
        for entry in entries.filter_map(Result::ok) {
            //check if entry is a dir
            if entry.file_type().unwrap().is_dir() {
                continue;
            }
            log!("Entry: {:?}", entry);
            //if count > 10 {
            //    break;
            //}
            //count += 1;
            let path = entry.path();
            let file_path = path.display().to_string();
            println!("file_path {:?}", file_path);
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if !matches!(ext.as_str(), "mp4" | "mkv" | "avi" | "mov") {
                    println!("Not a video SKIPPING");
                    continue;
                }
            }

            let metadata = match parse_exif(file_path.clone()).await {
                Ok(metadata) => metadata,
                Err(e) => {
                    println!("Error on {:?}", file_path);
                    println!("Error: {:?}", e);
                    continue;
                }
            };
            let usable_duration =
                if Some(metadata.duration_ms) != None && metadata.duration_ms.unwrap() < 5000 {
                    Some(false)
                } else {
                    Some(true)
                };
            let mut media_asset = Metadata {
                usable: "not processed".to_string(),
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
                root_path: root_path.to_string(),
                route: route.to_string(),
                file_path: media_asset.path.clone(),
                file_name: media_asset.file_name.clone(),
                media_type: media_asset.asset_type.clone(),
                usable: usable_duration,
                highlight: Option::from(false),
                reviewed: Option::from(false),
                duration_ms: metadata.duration_ms.unwrap(),
                created_at: DateTime::from(metadata.creation_date.unwrap()),
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
                    //log!("Inserting media asset: {:?}", media_asset);
                    //log!("Insert new media: {:?}", media_new);
                    insert_new_media(&media_new);
                }
            }
        }
    }

    files
}

pub async fn scan_original_paths() {
    let settings = settings();
    for path in &settings.paths {
        if let Some(orig_path) = &path.originals_path {
            let _ = update_original_paths(orig_path.as_str(), path.root_path.as_str()).await;

            // Scan subdirectories
            let dirs = find_dirs(orig_path.as_str());
            for dir in dirs {
                let _ = update_original_paths(dir.as_str(), path.root_path.as_str()).await;
            }
        }
    }
}

pub async fn update_original_paths(dir: &str, root_path: &str) -> Result<()> {
    log!("scanning original paths in: {}", dir);

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            if entry.file_type().unwrap().is_dir() {
                continue;
            }

            let path = entry.path();
            if let Some(file_name) = path.file_name().map(|n| n.to_string_lossy().to_string()) {
                // Update the database record that matches this filename and root_path
                update_media_original_path(
                    &file_name,
                    root_path,
                    path.to_string_lossy().to_string(),
                )?;
            }
        }
    }

    Ok(())
}
