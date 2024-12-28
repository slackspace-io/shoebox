use leptos::prelude::*;
use leptos::logging::log;
use leptos::prelude::{Get, GlobalAttributes, OnAttribute, Resource, RwSignal, Suspend, Suspense, Write};
use leptos::task::spawn_local;
use crate::app::{get_all_rows, get_files};
use crate::components::video_player::video_player;

#[component]
pub fn HomePage() -> impl IntoView {
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
                        //play vid
                        {video_player(video_url)}

                    }).unwrap_or_else(|| {
                        // Fallback must match successful branch structure
                        {video_player(fallback_video.to_string())}

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
