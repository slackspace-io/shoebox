use axum::{
    extract::{Path, State},
    routing::{get, post, delete},
    Json, Router,
};

use crate::error::Result;
use crate::models::CreateTagDto;
use crate::services::AppState;
use crate::services::TagService;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(list_tags))
        .route("/", post(create_tag))
        .route("/usage", get(get_tag_usage))
        .route("/cleanup", post(cleanup_unused_tags))
        .route("/{id}", get(get_tag))
        .route("/{id}", delete(delete_tag))
        .with_state(app_state)
}

async fn list_tags(State(state): State<AppState>) -> Result<Json<Vec<crate::models::Tag>>> {
    let tag_service = TagService::new(state.db.clone());
    let tags = tag_service.find_all().await?;
    Ok(Json(tags))
}

async fn get_tag(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<crate::models::Tag>> {
    let tag_service = TagService::new(state.db.clone());
    let tag = tag_service.find_by_id(&id).await?;
    Ok(Json(tag))
}

async fn create_tag(
    State(state): State<AppState>,
    Json(create_dto): Json<CreateTagDto>,
) -> Result<Json<crate::models::Tag>> {
    let tag_service = TagService::new(state.db.clone());
    let tag = tag_service.create(create_dto).await?;
    Ok(Json(tag))
}

async fn delete_tag(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<()>> {
    let tag_service = TagService::new(state.db.clone());
    tag_service.delete(&id).await?;
    Ok(Json(()))
}

async fn get_tag_usage(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::models::TagUsage>>> {
    let tag_service = TagService::new(state.db.clone());
    let usage = tag_service.get_usage().await?;
    Ok(Json(usage))
}

#[derive(serde::Serialize)]
struct CleanupResponse {
    count: usize,
}

async fn cleanup_unused_tags(State(state): State<AppState>) -> Result<Json<CleanupResponse>> {
    let tag_service = TagService::new(state.db.clone());
    let count = tag_service.cleanup_unused().await?;
    Ok(Json(CleanupResponse { count }))
}
