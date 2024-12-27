use leptos::logging::log;
use leptos::prelude::*;
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
                                <p>{format!("hi {:?}", file)}</p>
                                <video controls width="600" >
                                    <source src={video_url} type="video/mp4" />
                                    "Your browser does not support the video tag."
                                </video>
                            </div>
                        }
                    }).unwrap_or_else(|| {
                        // Fallback must match successful branch structure
                        view! {
                            <div>
                                <p>{format!("{:?}", "something")}</p>
                                <video controls width="600" >
                                    <source src={fallback_video.to_string()} type="video/mp4" />
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
            <Suspense>{contents}</Suspense>
        </div>
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
    log!("Getting files");
    use crate::filesystem::fs_watcher::scan_files;
    let files = scan_files("/home/dopey/videos").await; // Adjust based on actual API
    log!("Files gotten");
    use crate::database::return_all_media_assets;
    log!("Getting all media assets");
    let assets = return_all_media_assets().unwrap();
    log!("Media assets gotten");
    println!("{:?}", assets);
    Ok(assets)
}
