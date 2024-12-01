use axum::Router;
use axum::routing::get_service;
use http::StatusCode;
use tower_http::services::ServeDir;
use tracing::log;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use shoebox::app::*;
    use shoebox::fileserv::file_and_error_handler;
    use console_log::init_with_level;
    use log::Level;
    init_with_level(Level::Info).expect("Failed to initialize logger");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .nest_service("/videos", get_service(ServeDir::new("/home/dopey/videos")).handle_error(|_| async { (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error") }))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on https:///{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}


#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
