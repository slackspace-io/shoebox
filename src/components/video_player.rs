use leptos::prelude::*;

#[component]
pub fn VideoPlayer(video_url: String) -> impl IntoView {
    view! {
        <div>
            <video controls width="100%" src={video_url}>
                "Your browser does not support the video tag."
            </video>
        </div>
    }
}
