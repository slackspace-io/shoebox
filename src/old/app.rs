use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos::ev::MouseEvent;
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
            }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="files" view=ShowFiles/>
                    <Route path="db_files" view=ShowDBFiles/>
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
                        let file_name = path.split('/').last().unwrap();
                        let video_url = format!("/videos/{}", file_name);
                        view! {
                            <li>
                                <video controls width="600">
                                    <source src={video_url} type="video/mp4"/>
                                    "Your browser does not support the video tag."
                                </video>
                            </li>
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


#[cfg(feature = "ssr")]
use crate::db::db_calls::get_all_media_files;
#[cfg(feature = "ssr")]
use crate::db::models::MediaFile;

//#[server]
//pub async fn get_all_db_files() -> Result<Vec<MediaFile>, ServerFnError> {
//    let files = get_all_media_files();
//    Ok(files)
//}


#[component]
fn ShowDBFiles() -> impl IntoView {
    #[cfg(feature = "ssr")]
    let results = get_all_media_files();
    let (index, set_index) = create_signal(1);
    //let next_index = move |_| set_index.update(|index| *index += 1);
    #[cfg(feature = "ssr")]
    {results.get(index.get()).map(|result: &MediaFile| {
        view! {
//            <button on:click=next_index>"Next: " {index}</button>
            <li>"Id: " {&result.id.to_string()} "File: " {&result.name} " Path: " {&result.path}</li>
        }
    })}
    //view! {
    //    <button on:click=next_index>"Next: " {index}</button>

    //        //{results.get(index.get()).map(|result: MediaFile| {
    //        //    view! {
    //        //        <li>"Id: " {&result.id.to_string()} "File: " {&result.name} " Path: " {&result.path}</li>
    //        //    }

    //        //})}
    //}

    #[cfg(not(feature = "ssr"))]
    view! {
        <p>"Database access is not available."</p>
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
        <p>"Link to db files page: " <a href="/db_files">"DB Files"</a></p>
    }
}

