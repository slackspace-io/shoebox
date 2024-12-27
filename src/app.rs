use leptos::logging::log;
use leptos::prelude::*;
use gloo_timers::future::TimeoutFuture;
use leptos::task::spawn_local;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use crate::lib_models::FileType;
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
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePasge() -> impl IntoView {
    // Reactive signal for the counter
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    // Resource to fetch data asynchronously
    let res = Resource::new_blocking(
        || (),
        |_| async move { get_all_rows().await.unwrap() },
    );

    // Signal for the current video URL
    let current_video_url = RwSignal::new(String::new());

    let contents = move || {
        Suspend::new(async move {
            let data = res.await;
            // Placeholder video name for fallback
            let fallback_video = "test.mp4";

            let video_url = data.get(count.get()).map(|file| {
                let is_video = file.asset_type == "video";
                let file_name = file.path.split('/').last().unwrap_or_default();
                let video_url = if is_video {
                    format!("/videos/{}", file_name)
                } else {
                    file.path.clone()
                };
                // Update the video URL signal
                current_video_url.set(video_url.clone());
                video_url
            }).unwrap_or_else(|| fallback_video.to_string());

            view! {
                <div>
                    <p>{format!("hi {:?}", video_url)}</p>
                    // Bind the video URL dynamically
                    <video controls width="600" id={count.get()}>
                        <source src={video_url} type="video/mp4" />
                        "Your browser does not support the video tag."
                    </video>
                </div>
            }
        })
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>
            "Click Me: " {count}
        </button>
        <div>
            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
                {contents}
            </Suspense>
        </div>
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
                get_all_rows().await;
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

async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
        TimeoutFuture::new(1_000).await;
    //get file from medial file matching value number
    //log file
    value * 10
}



async fn get_latest_file(media_files: Vec<MediaFile>) -> String {
    //get latest file
    let latest_file = media_files[1].clone();
    //return path
    latest_file.path
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
pub async fn get_db_rows() -> Result<Vec<MediaFile>, ServerFnError> {
    use rusqlite::Connection;
    let conn = Connection::open("data.db")?;
    let mut stmt = conn.prepare("SELECT * FROM media_assets")?;
    let media_assets = stmt.query_map([], |row| {
        Ok(MediaFile {
            asset_type: row.get(1)?,
            path: row.get(2)?,
        })
    })?;
    let mut media_assets_vec = Vec::new();
    for media_asset in media_assets {
        media_assets_vec.push(media_asset?);
    }
    Ok(media_assets_vec)
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
    log!("First asset: {:?}", assets[0]);
    //log second asset
    log!("Second asset: {:?}", assets[1]);
   // println!("{:?}", assets);
    Ok(assets)
}
