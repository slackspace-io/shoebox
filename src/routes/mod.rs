mod video;
mod tag;
mod person;
mod scan;
mod export;
mod system;
mod location;
mod event;
mod shoebox;
pub mod media;

use axum::Router;

use crate::services::AppState;

pub fn api_router(app_state: AppState) -> Router {
    Router::new()
        // Video routes
        .nest("/videos", video::router(app_state.clone()))
        // Tag routes
        .nest("/tags", tag::router(app_state.clone()))
        // Person routes
        .nest("/people", person::router(app_state.clone()))
        // Location routes
        .nest("/locations", location::router(app_state.clone()))
        // Event routes
        .nest("/events", event::router(app_state.clone()))
        // Shoebox routes
        .nest("/shoeboxes", shoebox::router(app_state.clone()))
        // Scan routes
        .nest("/scan", scan::router(app_state.clone()))
        // Export routes
        .nest("/export", export::router(app_state.clone()))
        // System info routes
        .nest("/system", system::router(app_state))
}
