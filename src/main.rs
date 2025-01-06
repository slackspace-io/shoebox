use axum::routing::get_service;
use http::StatusCode;
use tower_http::services::ServeDir;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use shoebox::app::*;
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    //run get_files from App
    //let files = get_files().await.unwrap();
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .nest_service(
            "/videos-two",
            get_service(ServeDir::new("/mnt/storage/tove/immich/auto-transcoded/")).handle_error(
                |_| async { (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error") },
            ),
        )
        .nest_service(
            "/videos",
            get_service(ServeDir::new("/mnt/storage/tove/immich/auto-transcoded/")).handle_error(
                |_| async { (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error") },
            ),
        )
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    //connect to db
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
