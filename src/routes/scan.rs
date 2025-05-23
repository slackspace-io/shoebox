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

        // Start the scan but don't wait for it to complete
        match ScannerService::scan_directories(
            &source_paths,
            video_service,
            thumbnail_service,
            &config,
        ).await {
            Ok((new_videos_arc, updated_videos_arc, tasks)) => {
                // Spawn another task to wait for all processing tasks to complete
                // This ensures the main scan task returns quickly
                tokio::spawn(async move {
                    // Periodically update the scan status with progress
                    let update_interval = tokio::time::Duration::from_secs(2);
                    let mut interval = tokio::time::interval(update_interval);

                    // Track tasks that are still running
                    let mut remaining_tasks = tasks;

                    while !remaining_tasks.is_empty() {
                        interval.tick().await;

                        // Update status with current progress
                        {
                            let new_count = {
                                let guard = new_videos_arc.lock().await;
                                guard.len()
                            };

                            let updated_count = {
                                let guard = updated_videos_arc.lock().await;
                                guard.len()
                            };

                            let mut status = scan_status.write().await;
                            status.new_videos_count = new_count;
                            status.updated_videos_count = updated_count;
                        }

                        // Check which tasks have completed
                        remaining_tasks.retain(|task| !task.is_finished());
                    }

                    // All tasks completed, collect final results
                    match ScannerService::collect_scan_results(
                        new_videos_arc,
                        updated_videos_arc,
                        Vec::new() // Empty vec since we've already waited for tasks
                    ).await {
                        Ok((new_videos, updated_videos)) => {
                            // Update scan status with final results
                            let mut status = scan_status.write().await;
                            status.in_progress = false;
                            status.new_videos_count = new_videos.len();
                            status.updated_videos_count = updated_videos.len();
                        },
                        Err(e) => {
                            tracing::error!("Error collecting scan results: {}", e);
                            // Mark scan as not in progress even if it failed
                            let mut status = scan_status.write().await;
                            status.in_progress = false;
                        }
                    }
                });
            },
            Err(e) => {
                tracing::error!("Error starting scan: {}", e);
                // Mark scan as not in progress if it failed to start
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
