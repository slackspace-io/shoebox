use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::filesystem::fs_watcher::{scan_files, FileType};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/shoebox.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="files" view=ShowFiles/>
                </Routes>
            </main>
        </Router>
    }
}

///show files
#[component]
fn ShowFiles() -> impl IntoView {
    let files = scan_files("/home/dopey/videos");
    println!("Files: {:#?}", files);
    view! {
        <ul>
            {files.iter().map(|file| {
                match file {
                    FileType::Photo(path) => {
                        view! {
                            <li>"Photo: " {path}</li>
                        }
                    }
                    FileType::Video(path) => {
                        view! {
                            <li>"Video: " {path}</li>
                        }
                    }
                    FileType::Other(path) => {
                        view! {
                            <li>"Other: " {path}</li>
                        }
                    }
                }
            }).collect::<Vec<_>>()}
        </ul>
    }
}
/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <p>"This is a simple example of a Leptos application."</p>
        <p>"Link to files page: " <a href="/files">"Files"</a></p>
    }
}
