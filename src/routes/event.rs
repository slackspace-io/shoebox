use axum::{
    extract::{Path, State},
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::services::{AppState, EventService};

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_events))
        .route("/usage", get(get_event_usage))
        .route("/update", post(update_event))
        .route("/{event}", delete(delete_event))
        .with_state(app_state)
}

async fn get_all_events(
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>> {
    let event_service = EventService::new(state.db.clone());
    let events = event_service.get_all_events().await?;
    Ok(Json(events))
}

async fn get_event_usage(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::services::EventUsage>>> {
    let event_service = EventService::new(state.db.clone());
    let usage = event_service.get_event_usage().await?;
    Ok(Json(usage))
}

#[derive(Debug, Deserialize, Serialize)]
struct UpdateEventRequest {
    old_event: String,
    new_event: String,
}

async fn update_event(
    State(state): State<AppState>,
    Json(request): Json<UpdateEventRequest>,
) -> Result<Json<usize>> {
    let event_service = EventService::new(state.db.clone());
    let count = event_service.update_event(&request.old_event, &request.new_event).await?;
    Ok(Json(count))
}

async fn delete_event(
    State(state): State<AppState>,
    Path(event): Path<String>,
) -> Result<Json<usize>> {
    let event_service = EventService::new(state.db.clone());
    let count = event_service.delete_event(&event).await?;
    Ok(Json(count))
}
