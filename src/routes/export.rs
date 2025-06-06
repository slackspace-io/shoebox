use axum::{extract::State, routing::post, Json, Router};
use serde::Serialize;

use crate::error::Result;
use crate::models::ExportRequest;
use crate::services::AppState;
use crate::services::{ExportService, VideoService, TagService, PersonService, ThumbnailService};

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", post(export_videos))
        .with_state(app_state)
}

#[derive(Debug, Serialize)]
struct ExportResponse {
    export_path: String,
    video_count: usize,
}

async fn export_videos(
    State(state): State<AppState>,
    Json(request): Json<ExportRequest>,
) -> Result<Json<ExportResponse>> {
    let video_service = VideoService::new(
        state.db.clone(),
        TagService::new(state.db.clone()),
        PersonService::new(state.db.clone()),
        ThumbnailService::new(&state.config),
        crate::services::ShoeboxService::new(state.db.clone()),
    );

    let export_service = ExportService::new(
        state.config.clone(),
        video_service,
    );

    let export_path = export_service.export_videos(request.clone()).await?;

    let response = ExportResponse {
        export_path,
        video_count: request.video_ids.len(),
    };

    Ok(Json(response))
}
