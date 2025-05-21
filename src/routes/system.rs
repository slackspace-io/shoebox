use axum::{
    extract::State,
    routing::get,
    Json, Router,
};

use crate::services::AppState;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_system_info))
        .with_state(app_state)
}

async fn get_system_info(
    State(state): State<AppState>,
) -> Json<crate::config::Config> {
    // Simply return the configuration from the app state
    Json(state.config)
}
