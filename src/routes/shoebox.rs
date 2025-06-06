use axum::{
    extract::{Path, State},
    routing::{get, post, delete, put},
    Json, Router,
};

use crate::error::Result;
use crate::models::{CreateShoeboxDto, Shoebox};
use crate::services::AppState;
use crate::services::ShoeboxService;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(list_shoeboxes))
        .route("/", post(create_shoebox))
        .route("/usage", get(get_shoebox_usage))
        .route("/cleanup", post(cleanup_unused_shoeboxes))
        .route("/{id}", get(get_shoebox))
        .route("/{id}", put(update_shoebox))
        .route("/{id}", delete(delete_shoebox))
        .route("/{id}/videos", get(get_videos_in_shoebox))
        .route("/{id}/videos/{video_id}", put(add_video_to_shoebox))
        .route("/{id}/videos/{video_id}", delete(remove_video_from_shoebox))
        .with_state(app_state)
}

async fn list_shoeboxes(State(state): State<AppState>) -> Result<Json<Vec<Shoebox>>> {
    let shoebox_service = ShoeboxService::new(state.db.clone());
    let shoeboxes = shoebox_service.find_all().await?;
    Ok(Json(shoeboxes))
}

async fn get_shoebox(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Shoebox>> {
    let shoebox_service = ShoeboxService::new(state.db.clone());
    let shoebox = shoebox_service.find_by_id(&id).await?;
    Ok(Json(shoebox))
}

async fn create_shoebox(
    State(state): State<AppState>,
    Json(create_dto): Json<CreateShoeboxDto>,
) -> Result<Json<Shoebox>> {
    let shoebox_service = ShoeboxService::new(state.db.clone());
    let shoebox = shoebox_service.create(create_dto).await?;
    Ok(Json(shoebox))
}

#[derive(serde::Deserialize)]
struct UpdateShoeboxDto {
    name: String,
    description: Option<String>,
}

async fn update_shoebox(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(update_dto): Json<UpdateShoeboxDto>,
) -> Result<Json<Shoebox>> {
    let shoebox_service = ShoeboxService::new(state.db.clone());
    let shoebox = shoebox_service.update(&id, &update_dto.name, update_dto.description.as_deref()).await?;
    Ok(Json(shoebox))
}

async fn delete_shoebox(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<()>> {
    let shoebox_service = ShoeboxService::new(state.db.clone());
    shoebox_service.delete(&id).await?;
    Ok(Json(()))
}

async fn get_shoebox_usage(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::models::ShoeboxUsage>>> {
    let shoebox_service = ShoeboxService::new(state.db.clone());
    let usage = shoebox_service.get_usage().await?;
    Ok(Json(usage))
}

#[derive(serde::Serialize)]
struct CleanupResponse {
    count: usize,
}

async fn cleanup_unused_shoeboxes(State(state): State<AppState>) -> Result<Json<CleanupResponse>> {
    let shoebox_service = ShoeboxService::new(state.db.clone());
    let count = shoebox_service.cleanup_unused().await?;
    Ok(Json(CleanupResponse { count }))
}

async fn get_videos_in_shoebox(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<String>>> {
    let shoebox_service = ShoeboxService::new(state.db.clone());
    let video_ids = shoebox_service.get_videos_in_shoebox(&id).await?;
    Ok(Json(video_ids))
}

async fn add_video_to_shoebox(
    State(state): State<AppState>,
    Path((id, video_id)): Path<(String, String)>,
) -> Result<Json<()>> {
    let shoebox_service = ShoeboxService::new(state.db.clone());
    shoebox_service.add_video_to_shoebox(&video_id, &id).await?;
    Ok(Json(()))
}

async fn remove_video_from_shoebox(
    State(state): State<AppState>,
    Path((id, video_id)): Path<(String, String)>,
) -> Result<Json<()>> {
    let shoebox_service = ShoeboxService::new(state.db.clone());
    shoebox_service.remove_video_from_shoebox(&video_id, &id).await?;
    Ok(Json(()))
}
