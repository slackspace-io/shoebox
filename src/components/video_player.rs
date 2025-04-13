use crate::components::alert::{Alert, AlertDescription, AlertTitle};
use leptos::prelude::*;

#[component]
pub fn VideoPlayer(video_url: String) -> impl IntoView {
    view! {
        <div>
            <p>{format!("{:?}", video_url)}</p>
            <video controls width="600"
            src={video_url}
        >
                "Your browser does not support the video tag."
            </video>
        </div>
    }
}
