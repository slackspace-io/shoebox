use crate::components::metadata_form::VideoMetadataForm;
use crate::components::shadcn_card::{
    Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::lib_models::MediaWeb;
use leptos::either::Either;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::console::log;

#[component]
pub fn MediaCardReview(
    media_web: MediaWeb,
    tags: Option<Vec<String>>,
    people: Option<Vec<String>>,
    editable: bool,
) -> impl IntoView {
    let media_id = media_web.id.clone();
    let path = media_web.file_path.clone();
    let video_url = format!("{}/{}", media_web.route, media_web.relative_file_path());
    let file_name = media_web.file_name.clone();
    let file_name_no_ext = media_web.file_name_no_ext();
    let description = media_web.description.clone();
    let review_people = people.clone();
    let review_tags = tags.clone();
    let current_tags = tags.clone().unwrap_or(media_web.tags.clone());
    let current_people = people.clone().unwrap_or(media_web.people.clone());
    let media_type = media_web.media_type.clone();
    let reviewed = media_web.reviewed.clone();
    let created_at = media_web.created_at.clone();
    let uploaded_at = media_web.uploaded_at.clone();
    let highlight = media_web.highlight.clone();

    let card_class_string = if let Some(highlight) = highlight {
        if highlight {
            "w-fit place-content-center border-accent border-double"
        } else {
            "w-fit place-content-center"
        }
    } else {
        "w-fit place-content-center"
    };

    view! {
        <div>
            <Card class=card_class_string>
                <CardHeader>
                    <CardTitle>{file_name_no_ext}</CardTitle>
                    <CardDescription>{description}</CardDescription>
                </CardHeader>
                <CardContent class="grid gap-4">
                    <div class="flex items-center space-x-4 rounded-md border p-2">
                        <div class="flex-1 space-y-1">
                            <VideoPlayer video_url=video_url />
                            <p class="text-sm text-muted-foreground"></p>
                        </div>
                    </div>
                </CardContent>
                <CardFooter>
                    {if editable {
                        Either::Left(view! {
                            <div class="flex-row items-center">
                                <VideoMetadataForm file={file_name.clone()} tags={review_tags} people={review_people} />
                            </div>
                        })
                    } else {
                        Either::Right(view! {
                            <div></div>
                        })
                    }}
                </CardFooter>
            </Card>
        </div>
    }
}

#[server]
pub async fn remove_person(person: String, media_id: i32) -> Result<(), ServerFnError> {
    use crate::database::pg_deletes::remove_person;
    println!("Removing Person: {}", person);
    println!("Removing Person from media_id: {}", media_id);
    let removal = remove_person(media_id, person);
    match removal.await {
        Ok(_) => {
            log!("Person removed successfully");
        }
        Err(e) => {
            log!("{}", &format!("Error removing tag: {}", e));
        }
    }
    //reload page

    Ok(())
}

#[server]
pub async fn remove_tag(tag: String, media_id: i32) -> Result<(), ServerFnError> {
    use crate::database::pg_deletes::remove_tag;
    println!("Removing tag: {}", tag);
    println!("Removing tag from media_id: {}", media_id);
    let removal = remove_tag(media_id, tag);
    match removal.await {
        Ok(_) => {
            log!("Tag removed successfully");
        }
        Err(e) => {
            log!("{}", &format!("Error removing tag: {}", e));
        }
    }
    //reload page

    Ok(())
}

#[component]
pub fn VideoPlayer(video_url: String) -> impl IntoView {
    let video_url = video_url.clone();
    let class_url = video_url.clone();

    view! {
        <div>
            <video controls width=1200  src={class_url}>
                <source src={video_url} type="video/mp4" />
                Your browser does not support the video tag.
            </video>
        </div>
    }
}
