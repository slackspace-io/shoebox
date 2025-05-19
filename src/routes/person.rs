use axum::{
    extract::{Path, State},
    routing::{get, post, delete},
    Json, Router,
};

use crate::error::Result;
use crate::models::CreatePersonDto;
use crate::services::AppState;
use crate::services::PersonService;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(list_people))
        .route("/", post(create_person))
        .route("/usage", get(get_person_usage))
        .route("/cleanup", post(cleanup_unused_people))
        .route("/:id", get(get_person))
        .route("/:id", delete(delete_person))
        .with_state(app_state)
}

async fn list_people(State(state): State<AppState>) -> Result<Json<Vec<crate::models::Person>>> {
    let person_service = PersonService::new(state.db.clone());
    let people = person_service.find_all().await?;
    Ok(Json(people))
}

async fn get_person(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<crate::models::Person>> {
    let person_service = PersonService::new(state.db.clone());
    let person = person_service.find_by_id(&id).await?;
    Ok(Json(person))
}

async fn create_person(
    State(state): State<AppState>,
    Json(create_dto): Json<CreatePersonDto>,
) -> Result<Json<crate::models::Person>> {
    let person_service = PersonService::new(state.db.clone());
    let person = person_service.create(create_dto).await?;
    Ok(Json(person))
}

async fn delete_person(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<()>> {
    let person_service = PersonService::new(state.db.clone());
    person_service.delete(&id).await?;
    Ok(Json(()))
}

async fn get_person_usage(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::models::PersonUsage>>> {
    let person_service = PersonService::new(state.db.clone());
    let usage = person_service.get_usage().await?;
    Ok(Json(usage))
}

#[derive(serde::Serialize)]
struct CleanupResponse {
    count: usize,
}

async fn cleanup_unused_people(State(state): State<AppState>) -> Result<Json<CleanupResponse>> {
    let person_service = PersonService::new(state.db.clone());
    let count = person_service.cleanup_unused().await?;
    Ok(Json(CleanupResponse { count }))
}
