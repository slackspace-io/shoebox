use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Shoebox Home"</h1>
        <br />
        <a href="/browse">"Browse"</a>
        <br />
        <a href="/review">"Review"</a>

    }
}
