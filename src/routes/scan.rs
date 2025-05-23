use axum::{extract::State, routing::{post, get}, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::Result;
use crate::services::AppState;
use crate::services::{ScannerService, VideoService, ThumbnailService, TagService, PersonService, ScanStatus};

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", post(start_scan))
        .route("/status", get(get_scan_status))
        .with_state(app_state)
}

#[derive(Debug, Serialize)]
struct ScanResponse {
    message: String,
    scan_started: bool,
}

#[derive(Debug, Serialize)]
struct ScanStatusResponse {
    in_progress: bool,
    new_videos_count: usize,
    updated_videos_count: usize,
}

async fn start_scan(State(state): State<AppState>) -> Result<Json<ScanResponse>> {
    // Check if a scan is already in progress
    let is_scan_in_progress = {
        let status = state.scan_status.read().await;
        status.in_progress
    };

    if is_scan_in_progress {
        return Ok(Json(ScanResponse {
            message: "A scan is already in progress".to_string(),
            scan_started: false,
        }));
    }

    // Mark scan as in progress
    {
        let mut status = state.scan_status.write().await;
        status.in_progress = true;
        status.new_videos_count = 0;
        status.updated_videos_count = 0;
    }

    // Clone what we need for the background task
    let db = state.db.clone();
    let config = state.config.clone();
    let scan_status = state.scan_status.clone();
    let source_paths = config.media.source_paths.clone();

    // Spawn a background task to perform the scan
    tokio::spawn(async move {
        let video_service = VideoService::new(
            db.clone(),
            TagService::new(db.clone()),
            PersonService::new(db.clone()),
            ThumbnailService::new(&config),
        );

        let thumbnail_service = ThumbnailService::new(&config);

        // Scan directories
        match ScannerService::scan_directories(
            &source_paths,
            video_service,
            thumbnail_service,
        ).await {
            Ok((new_videos, updated_videos)) => {
                // Update scan status
                let mut status = scan_status.write().await;
                status.in_progress = false;
                status.new_videos_count = new_videos.len();
                status.updated_videos_count = updated_videos.len();
            },
            Err(e) => {
                tracing::error!("Error during scan: {}", e);
                // Mark scan as not in progress even if it failed
                let mut status = scan_status.write().await;
                status.in_progress = false;
            }
        }
    });

    Ok(Json(ScanResponse {
        message: "Scan started successfully".to_string(),
        scan_started: true,
    }))
}

async fn get_scan_status(State(state): State<AppState>) -> Result<Json<ScanStatusResponse>> {
    let status = state.scan_status.read().await;

    Ok(Json(ScanStatusResponse {
        in_progress: status.in_progress,
        new_videos_count: status.new_videos_count,
        updated_videos_count: status.updated_videos_count,
    }))
}
