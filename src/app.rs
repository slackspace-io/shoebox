use crate::components::shadcn_button::{Button, ButtonVariant};
use crate::lib_models::{FileType, MediaFile, Metadata};
use crate::pages::browse::BrowsePage;
use crate::pages::homepage::HomePage;
use crate::pages::review::{ReviewPage, ReviewReload};
use crate::pages::search::SearchPage;
use crate::pages::test_form::FormExample;
use gloo_timers::future::TimeoutFuture;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::Form;
use leptos_router::hooks::use_query_map;
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" >
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-background">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    //load files

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/shoebox.css"/>

        // sets the document title
        <Title text="Shoebox"/>

        // content for this welcome page
        <Router>
        <nav class="place-items-center">
        <div>
        <Button variant={ButtonVariant::Link}>
        <a href="/">Home</a>
        </Button>
        <Button variant={ButtonVariant::Link}>
            <a href="/browse">Browse</a>
        </Button>
        <Button variant={ButtonVariant::Link}>
            <a href="/review">Review</a>
        </Button>
        <Button variant={ButtonVariant::Link}>
            <a href="/search">Search</a>
        </Button>
        </div>
        </nav>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=path!("/review") view=ReviewPage/>
                    <Route path=path!("/browse") view=BrowsePage/>
                    <Route path=path!("/test") view=FormExample/>
                    <Route path=path!("/search") view=SearchPage/>
                    <Route path=path!("/review/next") view=ReviewReload/>
                </Routes>
            </main>
        </Router>
    }
}

#[server]
pub async fn get_files() -> Result<(), ServerFnError> {
    use crate::filesystem::fs_watcher::scan_all;
    use crate::filesystem::fs_watcher::scan_original_paths;
    log!("Getting files");
    let files = scan_all().await;
    let _ = scan_original_paths().await;
    log!("Files: {:?}", files);
    Ok(())
}
