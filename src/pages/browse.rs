use crate::components::media_card::MediaCard;
use crate::components::shadcn_button::Button;
use crate::components::shadcn_card::{
    Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::lib_models::{MediaWeb, VideoMetadata};
use leptos::attr::controls;
use leptos::html::{video, Video};
use leptos::logging::log;
use leptos::prelude::*;
use lucide_leptos::{BellRing, Check};

#[component]
pub fn BrowsePage() -> impl IntoView {
    //get files
    //let files = Resource::new_blocking(
    //    || (),
    //    |_| async move {get_files().await.unwrap() },
    //);
    let files = Resource::new_blocking(
        || (),
        |_| async move { get_all_media_assets().await.unwrap() },
    );
    let fallback_message = &String::from("No files found");
    //hello world
    view! {
    <div class="place-items-center">
    <Suspense
    fallback= move || {
        view! {
            <p>"Loading..."</p>
        }
    }>
    //list files
    <div class="grid grid-cols-4 gap-4">
        {move || files.get().iter().next().map(|file| {
            view! {
                    {file.iter().map(|f| {

                        view! {
                            <MediaCard media_web = f.clone() tags=None people=None editable = false/>
                        }
                    }).collect::<Vec<_>>()}
            }
        })}
    </div>

    </Suspense>
    </div>
    }
}

#[server]
pub async fn get_all_media_assets() -> Result<Vec<MediaWeb>, ServerFnError> {
    use crate::database::pg_calls::fetch_video_assets;
    let assets = fetch_video_assets(false).await;
    if let Ok(assets) = assets {
        Ok(assets)
    } else {
        Err(ServerFnError::new("Error fetching media assets"))
    }
}
