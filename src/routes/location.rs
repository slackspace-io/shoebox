use axum::{
    extract::{Path, State},
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::services::{AppState, LocationService};

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_locations))
        .route("/usage", get(get_location_usage))
        .route("/update", post(update_location))
        .route("/{location}", delete(delete_location))
        .with_state(app_state)
}

async fn get_all_locations(
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>> {
    let location_service = LocationService::new(state.db.clone());
    let locations = location_service.get_all_locations().await?;
    Ok(Json(locations))
}

async fn get_location_usage(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::services::LocationUsage>>> {
    let location_service = LocationService::new(state.db.clone());
    let usage = location_service.get_location_usage().await?;
    Ok(Json(usage))
}

#[derive(Debug, Deserialize, Serialize)]
struct UpdateLocationRequest {
    old_location: String,
    new_location: String,
}

async fn update_location(
    State(state): State<AppState>,
    Json(request): Json<UpdateLocationRequest>,
) -> Result<Json<usize>> {
    let location_service = LocationService::new(state.db.clone());
    let count = location_service.update_location(&request.old_location, &request.new_location).await?;
    Ok(Json(count))
}

async fn delete_location(
    State(state): State<AppState>,
    Path(location): Path<String>,
) -> Result<Json<usize>> {
    let location_service = LocationService::new(state.db.clone());
    let count = location_service.delete_location(&location).await?;
    Ok(Json(count))
}
