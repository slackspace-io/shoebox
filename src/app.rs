use leptos::logging::log;
use leptos::prelude::*;
use gloo_timers::future::TimeoutFuture;
use leptos::task::spawn_local;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_router::components::Form;
use leptos_router::hooks::use_query_map;
use crate::lib_models::{FileType, Metadata};
use crate::models::MediaFile;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
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
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("form") view=FormExample/>
                </Routes>
            </main>
        </Router>
    }
}


#[component]
fn HomePage() -> impl IntoView {
    // Reactive signal for the counter
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    // Resource to fetch data asynchronously
    let res = Resource::new_blocking(
        || (),
        |_| async move { get_all_rows().await.unwrap() },
    );

    let contents = move || {
        Suspend::new(async move {
            let data = res.await;
            //placeholder video name for fallback
            let fallback_video = "test.mp4";
            view! {
                    // Display the current count
                    {log!("Count: {:?}", count.get())}
                    // Dynamically fetch the file based on the current count
                    {data.get(count.get()).map(|file| {
                        let is_video = file.asset_type == "video";
                        let file_name = file.path.split('/').last().unwrap_or_default();
                        let video_url = if is_video {
                            format!("/videos/{}", file_name)
                        } else {
                            file.path.clone()
                        };
                        log!("Video URL: {:?}", video_url);
                        log!("File: {:?}", file);
                        view! {
                            <div>
                                <p>{format!("hi {:?}", video_url)}</p>
                                <video controls width="600"
                                src={video_url} id={count.get()}
                            >
                                    "Your browser does not support the video tag."
                                </video>
                            </div>
                        }
                    }).unwrap_or_else(|| {
                        // Fallback must match successful branch structure
                        view! {
                            <div>
                                <p>{format!("{:?}", "something")}</p>
                                <video controls width="600"
                                    src={fallback_video.to_string()}  id={count.get()}
                            >
                                    "Your browser does not support the video tag."
                                </video>
                            </div>
                        }
                    })}
            }
        })
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>
            "Click Me: " {count}
        </button>
        <button on:click=move |_| {
            spawn_local(async {
                get_files().await;
            });
        }>
            "Get Files"
        </button>
        <div>
            <Suspense
                    fallback=move || view! { <p>"Loading..."</p> }
        >
        {contents}</Suspense>
        </div>
    }
}
#[component]
pub fn FormExample() -> impl IntoView {
    // reactive access to URL query
    let query = use_query_map();
    let name = move || query.read().get("name").unwrap_or_default();
    let number = move || query.read().get("number").unwrap_or_default();
    let select = move || query.read().get("select").unwrap_or_default();

    view! {
        // read out the URL query strings
        <table>
            <tr>
                <td><code>"name"</code></td>
                <td>{name}</td>
            </tr>
            <tr>
                <td><code>"number"</code></td>
                <td>{number}</td>
            </tr>
            <tr>
                <td><code>"select"</code></td>
                <td>{select}</td>
            </tr>
        </table>
        // <Form/> will navigate whenever submitted
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="">
            // input names determine query string key
            <input type="text" name="name" value=name/>
            <input type="number" name="number" value=number/>
            <select name="select">
                // `selected` will set which starts as selected
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
        // This <Form/> uses some JavaScript to submit
        // on every input

    }
}



#[server]
pub async fn get_files() -> Result<Vec<FileType>, ServerFnError> {
    // If scan_files returns a Vec<FileType> directly:
    use crate::filesystem::fs_watcher;
    use crate::filesystem::fs_watcher::scan_files;
    log!("Getting files");
    let files = scan_files("/home/dopey/videos").await; // Adjust based on actual API
    println!("{:?}", files);
    Ok(files)
}

#[server]
pub async fn get_all_rows() -> Result<Vec<MediaFile>, ServerFnError> {
    //log
    //log!("Getting files");
    //use crate::filesystem::fs_watcher::scan_files;
    //let files = scan_files("/home/dopey/videos").await; // Adjust based on actual API
    //log!("Files gotten");
    use crate::database::return_all_media_assets;
    log!("Getting all media assets");
    let assets = return_all_media_assets()?;
    log!("Media assets gotten");
    //log first asset
    //log second asset
   // println!("{:?}", assets);
    Ok(assets)
}
