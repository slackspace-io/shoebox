use leptos::prelude::*;
use leptos::logging::log;
use leptos::prelude::{Get, GlobalAttributes, OnAttribute, Resource, RwSignal, Suspend, Suspense, Write};
use leptos::task::spawn_local;
use leptos_router::components::Redirect;
use crate::app::{get_all_rows, get_files};
use crate::components::video_player::VideoPlayer;
use crate::components::metadata_form::VideoMetadataForm;

#[component]
pub fn ReviewReload() -> impl IntoView {
//Hack to reload after form submission
    view! {
<Redirect path="/review"/>
}
}


#[component]
pub fn HomePage() -> impl IntoView {
    // Reactive signal for the counter
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    let current_file = RwSignal::new("");
    // Resource to fetch data asynchronously
    let res = Resource::new_blocking(
        || (),
        |_| async move { get_all_rows().await.unwrap() },
    );

    let contents = move || {
        Suspend::new(async move {
            let data = res.await;
            //Total number of files
            let total_files = data.len();
            //placeholder video name for fallback
            let fallback_video = "test.mp4";
            //set current file
            *current_file.write() = fallback_video;
            data.get(count.get()).map(|file| {
                let is_video = file.asset_type == "video";
                let file_name = file.path.split('/').last().unwrap_or_default();
                let mut video_url = if is_video {
                    format!("/videos/{}", file_name)
                } else {
                    file.path.clone()
                };
                log!("Video URL: {:?}", video_url);
                log!("File: {:?}", file);
                //set current_file
                //play vid
                view! {
                <p>"Total Files: " {total_files}</p>
                   <VideoPlayer video_url=video_url/>
                   <VideoMetadataForm file=file.path.clone()/>
                }

            }).unwrap()
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
        {contents}
        </Suspense>
        </div>
    }
}
