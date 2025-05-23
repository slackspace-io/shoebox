use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::services::AppState;
use crate::services::{ScannerService, VideoService, ThumbnailService, TagService, PersonService};

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", post(scan_directories))
        .with_state(app_state)
}

#[derive(Debug, Serialize)]
struct ScanResponse {
    new_videos_count: usize,
    new_videos: Vec<crate::models::Video>,
    updated_videos_count: usize,
    updated_videos: Vec<crate::models::Video>,
}

async fn scan_directories(State(state): State<AppState>) -> Result<Json<ScanResponse>> {
    let video_service = VideoService::new(
        state.db.clone(),
        TagService::new(state.db.clone()),
        PersonService::new(state.db.clone()),
        ThumbnailService::new(&state.config),
    );

    let thumbnail_service = ThumbnailService::new(&state.config);

    // Get source paths from config
    let source_paths = state.config.media.source_paths.clone();

    // Scan directories
    let (new_videos, updated_videos) = ScannerService::scan_directories(
        &source_paths,
        &video_service,
        &thumbnail_service,
    ).await?;

    let response = ScanResponse {
        new_videos_count: new_videos.len(),
        new_videos,
        updated_videos_count: updated_videos.len(),
        updated_videos,
    };

    Ok(Json(response))
}
