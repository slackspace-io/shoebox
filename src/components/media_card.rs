// src/components/media_card.rs
use crate::components::{
    metadata_form::VideoMetadataForm,
    remove_handlers::{remove_person, remove_tag},
    shadcn_card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
    video_player::VideoPlayer,
};
use crate::lib_models::MediaWeb;
use leptos::either::Either;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn MediaCard(
    media_web: MediaWeb,
    tags: Option<Vec<String>>,
    people: Option<Vec<String>>,
    editable: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let media_id = media_web.id;
    let video_url = format!("{}/{}", media_web.route, media_web.relative_file_path());
    let file_name = media_web.file_name.clone();
    let file_name_no_ext = media_web.file_name_no_ext();
    let description = media_web.description;
    let current_tags = tags.as_ref().unwrap_or(&media_web.tags).clone();
    let current_people = people.as_ref().unwrap_or(&media_web.people).clone();
    let highlight = media_web.highlight.unwrap_or(false);

    let card_class = if highlight {
        "w-fit place-content-center border-accent border-double"
    } else {
        "w-fit place-content-center"
    };

    view! {
        <div>
            <Card class=card_class>
                                    <CardHeader>
                                        <CardTitle>{file_name_no_ext}</CardTitle>
                                        <CardDescription>{description}</CardDescription>
                                    </CardHeader>
                                    <CardContent class="grid gap-4">
                    <div class="flex items-center space-x-4 rounded-md border p-2">
                                            <div class="flex-1 space-y-1">
                            <VideoPlayer video_url=video_url />
                                            </div>
                                        </div>
    <div class="snap-center flex items-center">
      <h2 class="inline text-cyan-500 font-extrabold mr-2">Tags:</h2>
      <ul class="inline list-none p-0 m-0 flex gap-2">
        {current_tags.into_iter().map(|tag| {
                                let tag_clone = tag.clone();
          view! {
                                    <Card class="relative flex items-center bg-secondary border border-secondary rounded-full px-1.5 py-0.5 text-gray-800">
          <button
            class="absolute top-0 left-0.5 text-sm text-accent bg-transparent border-0 cursor-pointer"
                                on:click=move |_| {
                                                let tag = tag_clone.clone();
                                                let media_id = media_id.clone();
                                                spawn_local(async move {
                                                    remove_tag(tag, media_id).await.unwrap();
                                    });
                                }
            aria-label="Remove tag"
          >
                                            "x"
              </button>
                                        <span class="pl-3 pr-1">{tag}</span>
            </Card>
          }
        }).collect_view()}
      </ul>
    </div>
    <div class="snap-center flex items-center">
      <h2 class="inline text-cyan-500 font-extrabold mr-2">People:</h2>
      <ul class="inline list-none p-0 m-0 flex gap-2">
        {current_people.into_iter().map(|person| {
                                let person_clone = person.clone();
          view! {
            <Card class="relative flex items-center bg-secondary border border-secondary rounded-full px-1.5 py-0.5 text-gray-800">
          <button
            class="absolute top-0 left-0.5 text-sm text-accent bg-transparent border-0 cursor-pointer"
                                on:click=move |_| {
                                                let person = person_clone.clone();
                                                let media_id = media_id.clone();
                                    spawn_local(async move {
                                                    remove_person(person, media_id).await.unwrap();
                                    });
                                }
                                            aria-label="Remove person"
          >
                                            "x"
              </button>
                                        <span class="pl-3 pr-1">{person}</span>
            </Card>
          }
        }).collect_view()}
      </ul>
    </div>
                                    </CardContent>
                                    <CardFooter>
                                {if editable {
                        Either::Left(view! {
                                                <div class="flex-row items-center">
                                <VideoMetadataForm file=file_name tags=tags people=people />
                                                    </div>
                                    })
                                    } else {
                        Either::Right(view! { <div></div> })
                    }}
                    {children.map(|children| children())}
                                    </CardFooter>
                                </Card>
                                </div>
                            }
}
