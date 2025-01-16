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
pub fn MediaCard(
    media_web: MediaWeb,
    tags: Option<Vec<String>>,
    people: Option<Vec<String>>,
    editable: bool,
) -> impl IntoView {
    let media_id = media_web.id.clone();
    let path = media_web.file_path.clone();
    let video_url = format!("{}/{}", media_web.route, media_web.relative_file_path());
    println!("videl url {:?}", video_url);
    let file_name = media_web.file_name.clone();
    let file_name_no_ext = media_web.file_name_no_ext();
    let description = Some(media_web.description.clone());
    let current_tags = match tags {
        Some(tags) => tags,
        None => media_web.tags.clone(),
    };
    let current_people = match people {
        Some(people) => people,
        None => media_web.people.clone(),
    };
    //    let current_tags = media_web.tags.clone();
    //    let people = media_web.people.clone();
    let media_type = media_web.media_type.clone();
    let reviewed = media_web.reviewed.clone();
    let created_at = media_web.created_at.clone();
    let uploaded_at = media_web.uploaded_at.clone();
    let highlight = media_web.highlight.clone();
    let card_class_string = if highlight.expect("REASON") {
        "w-fit place-content-center border-accent border-double"
    } else {
        "w-fit place-content-center"
    };

    view! {
                                <div>
                                <Card class=card_class_string >
                                    <CardHeader>
                                        <CardTitle>{file_name_no_ext}</CardTitle>
                                        <CardDescription>{description}</CardDescription>
                                    </CardHeader>
                                    <CardContent class="grid gap-4">
                                        <div class=" flex items-center space-x-4 rounded-md border p-2">
                                            <div class="flex-1 space-y-1">
                                            <VideoPlayer video_url=video_url/>
                                                <p class="text-sm text-muted-foreground ">
                                                </p>
                                            </div>

                                        </div>
    <div class="snap-center flex items-center">
      <h2 class="inline text-cyan-500 font-extrabold mr-2">Tags:</h2>
      <ul class="inline list-none p-0 m-0 flex gap-2">
        {current_tags.into_iter().map(|tag| {
            let shownTag = tag.clone();
          view! {
            <Card class="relative flex items-center  border border-secondary rounded-full px-1.5 py-0.5 text-text bg-secondary">
              {/* 'X' link */}
          <button
            class="absolute top-0 left-0.5 text-sm text-accent bg-transparent border-0 cursor-pointer"
                                on:click=move |_| {
                                    // Call the server function with the tag to remove
                                       let tagClone = tag.clone();
                                        let media_id_tag = media_id.clone();
                                    spawn_local(async move {
                                        // Await the server function call
                                        remove_tag(tagClone, media_id_tag).await.unwrap();

                        //refresh page
                                    });
                                }
            aria-label="Remove tag"
          >
                x
              </button>
              <span class="pl-3 pr-1">{shownTag}</span>
            </Card>
          }
        }).collect_view()}
      </ul>
    </div>



    <div class="snap-center flex items-center">
      <h2 class="inline text-cyan-500 font-extrabold mr-2">People:</h2>
      <ul class="inline list-none p-0 m-0 flex gap-2">
        {current_people.into_iter().map(|person| {
            let shownPerson = person.clone();
          view! {
            <Card class="relative flex items-center bg-secondary border border-secondary rounded-full px-1.5 py-0.5 text-gray-800">
              {/* 'X' link */}
          <button
            class="absolute top-0 left-0.5 text-sm text-accent bg-transparent border-0 cursor-pointer"
                                on:click=move |_| {
                                    // Call the server function with the tag to remove
                                       let personClone = person.clone();
                                        let media_id_person = media_id.clone();
                                    spawn_local(async move {
                                        // Await the server function call
                                        remove_person(personClone, media_id_person).await.unwrap();

                        //refresh page
                                    });
                                }
            aria-label="Remove tag"
          >
                x
              </button>
              <span class="pl-3 pr-1">{shownPerson}</span>
            </Card>
          }
        }).collect_view()}
      </ul>
    </div>

                                        <div>

                                        </div>
                                    </CardContent>
                                    <CardFooter>
                                {if editable {
                                    Either::Left(view!{

                                                <div class="flex-row items-center">
                                                <VideoMetadataForm file={file_name.clone()} />

                                                    </div>
                                    })
                                    } else {
                                    Either::Right(view!{
                                        <div></div>
                                    })
                                }
                                }

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
