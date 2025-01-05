use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn HomePage() -> impl IntoView {
    let get_files = ServerAction::<LoadFiles>::new();
    let fetch_files = move |_| {
        spawn_local(async {
            load_files().await;
        });
    };

    view! {
        <h1>"Shoebox Home"</h1>
        <br />
        <a href="/browse">"Browse"</a>
        <br />
        <a href="/review">"Review"</a>
        <button on:click=fetch_files>"Refresh Files"</button>  // Button to trigger the action



    }
}

#[server]
pub async fn load_files() -> Result<String, ServerFnError> {
    use crate::app::get_files;
    get_files().await?;
    Ok("Done".to_string())
}
