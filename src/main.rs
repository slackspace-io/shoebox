use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use http::StatusCode;
use shoebox::database::pg_conn::pg_connection;
use shoebox::settings::settings;
mod immich;
mod settings;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    let settings = settings();
    println!("{settings:?}");
    // Print out our settings
    let mut connection = pg_connection();
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    //run migrations
    use axum::routing::get_service;
    use axum::Router;
    use leptos::{logging::log, prelude::*};
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use shoebox::app::*;
    use tower_http::services::ServeDir;
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    //run get_files from App
    //let files = get_files().await.unwrap();

    let mut app = Router::new();
    for path in &settings.paths {
        let route = format!("/{}", path.route(&path.root_path));
        println!("Route {:?}", route);
        let service = get_service(ServeDir::new(&path.root_path)).handle_error(|_| async {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        });
        app = app.nest_service(route.as_str(), service);
    }

    let app = app
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
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
