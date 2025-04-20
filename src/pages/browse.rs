use crate::components::media_card::MediaCard;
use crate::components::shadcn_button::Button;
use crate::components::shadcn_card::{
    Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::lib_models::{MediaWeb, VideoMetadata};
use leptos::attr::controls;
use leptos::ev::MouseEvent;
use leptos::html::{video, Video};
use leptos::logging::log;
use leptos::prelude::*;
use lucide_leptos::{BellRing, Check};

#[server]
pub async fn reset_for_review(media_id: i32) -> Result<(), ServerFnError> {
    use crate::database::pg_calls::reset_media_review_status;
    match reset_media_review_status(media_id) {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::new(format!(
            "Failed to reset review status: {}",
            e
        ))),
    }
}

#[component]
pub fn MediaCardWithReset(
    media_web: MediaWeb,
    tags: Option<Vec<String>>,
    people: Option<Vec<String>>,
    editable: bool,
) -> impl IntoView {
    let reset_action = create_action(|input: &ResetForReview| reset_for_review(input.media_id));

    let on_reset = Callback::new(move |_: MouseEvent| {
        reset_action.dispatch(ResetForReview {
            media_id: media_web.id,
        });
    });

    let reset_button = view! {
        <Button class="mt-2" onclick=on_reset>"Reset for Review"</Button>
    };

    view! {
        <MediaCard
            media_web=media_web.clone()
            tags=tags
            people=people
            editable=editable
        >
            {reset_button}
        </MediaCard>
    }
}

// Update the BrowsePage component to use MediaCardWithReset
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
                                    <MediaCardWithReset
                                        media_web=f.clone()
                                        tags=None
                                        people=None
                                        editable=false
                                    />
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
