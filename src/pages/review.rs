use leptos::attr::formaction;
use leptos::either::Either;
use leptos::ev::MouseEvent;
use leptos::html::video;
use leptos::logging::log;
use leptos::prelude::*;
use lucide_leptos::{BellRing, Check};
use crate::components::media_card::MediaCard;
use crate::components::metadata_form::VideoMetadataForm;
use crate::components::shadcn_button::{Button, ButtonVariant};
use crate::components::shadcn_card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::lib_models::{MediaWeb, VideoMetadata};






#[component]
pub fn ReviewPage() -> impl IntoView {
    let count = RwSignal::new(0);
//    let on_click = move |_| *count.write() += 1;
    let on_click = Callback::new(move |_: MouseEvent| {
        *count.write() += 1;
    });
    let files = Resource::new_blocking(
        || (),
        |_| async move {get_all_media_assets().await.unwrap() },
    );
    let fallback_message = &String::from("No files found");
    view! {
    <div class="place-items-center">
                <Button variant={ButtonVariant::Outline} onclick={on_click}>
        Next File:
        {count}
    </Button>

    <Suspense
    fallback= move || {
        view! {
            <p>"Loading..."</p>
        }
    }>
    //list files
        <div>
    {move || {
        files.get().iter().next().and_then(|file| {
            file.get(count.get()).map(|f| {
                Either::Left(view! {
                    <div>
                        <MediaCard media_web={f.clone()} editable=true />
                    </div>
                })
            })
        }).unwrap_or_else(|| {
            log!("No files found");
            Either::Right(FallbackView())
        })
    }}
        </div>

    </Suspense>
    </div>
    }
}


#[component]
pub fn FallbackView() -> impl IntoView {
    view! {
        <p>"No files found"</p>
        <a href="/">"Go Home"</a>
    }
}

#[server]
pub async fn get_all_media_assets() -> Result<Vec<MediaWeb>, ServerFnError> {
    use crate::database::pg_calls::fetch_video_assets;
    let assets = fetch_video_assets(true).await;
    if let Ok(assets) = assets {
        Ok(assets)
    } else {
        Err(ServerFnError::new("Error fetching media assets"))
    }
}


