mod config;
mod error;
mod models;
mod routes;
mod services;
mod utils;
mod db;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::Config::load()?;

    //log that db init will happen
    info!("DB Init");
    // Initialize database
    let db_pool = db::init_db(&config).await?;

    // Create application state
    let app_state = services::AppState {
        db: db_pool,
        config: config.clone(),
    };

    // Determine the path to the frontend dist directory
    let frontend_path = std::env::var("FRONTEND_PATH").unwrap_or_else(|_| {
        if std::path::Path::new("/app/frontend/dist").exists() {
            "/app/frontend/dist".to_string()
        } else {
            "frontend/dist".to_string()
        }
    });

    info!("Serving frontend from: {}", frontend_path);

    // Build our application with routes
    let app = Router::new()
        // API routes
        .nest("/api", routes::api_router(app_state.clone()))
        // Serve thumbnails from the thumbnails directory
        .nest_service("/app/thumbnails", ServeDir::new(&config.media.thumbnail_path))
        // Serve media files from the media directory with custom handler
        .nest("/media", routes::media::router(app_state))
        // Fallback for serving static files and SPA client-side routing
        .fallback_service(ServeDir::new(&frontend_path).append_index_html_on_directories(true));

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Listening on {}", addr);

    // Start the server
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
