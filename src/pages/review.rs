use leptos::attr::formaction;
use leptos::either::Either;
use leptos::ev::MouseEvent;
use leptos::html::{video};
use leptos::logging::log;
use leptos::prelude::*;
use lucide_leptos::{BellRing, Check};
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
pub fn VideoCard(files: Resource::<Vec<MediaWeb>>, count: RwSignal<usize>) -> impl IntoView {
    {move || {
        files.get().iter().next().and_then(|file| {
            file.get(count.get()).map(|f| {
                view! {
                    <div>
                        <MediaCard media_web={f.clone()} editable = true />
                    </div>
                };
            })
        }).unwrap_or_else(|| {
            view! {
                <div>
                    <p>"No media available."</p>
                </div>
            };
        })
    }}
}

#[component]
pub fn FallbackView() -> impl IntoView {
    view! {
        <p>"No files found"</p>
        <a href="/">"Go Home"</a>
    }
}



struct Notification {
    id: usize,
    title: &'static str,
    description: &'static str,
}

fn notifications() -> Vec<Notification> {
    vec![
        Notification {
            id: 0,
            title: "Your call has been confirmed.",
            description: "1 hour ago",
        },
        Notification {
            id: 1,
            title: "You have a new message!",
            description: "1 hour ago",
        },
        Notification {
            id: 2,
            title: "Your subscription is expiring soon!",
            description: "2 hours ago",
        },
    ]
}

#[component]
pub fn MediaCard(media_web: MediaWeb, editable: bool) -> impl IntoView {
    let path = media_web.file_path.clone();
    let video_url = format!("/videos/{}", media_web.file_name);
    let file_name = media_web.file_name.clone();
    let file_name_no_ext = media_web.file_name_no_ext();
    let description = Some(media_web.description.clone());
    let tags = media_web.tags.clone();
    let people = media_web.people.clone();
    let media_type = media_web.media_type.clone();
    let reviewed = media_web.reviewed.clone();
    let created_at = media_web.created_at.clone();
    let uploaded_at = media_web.uploaded_at.clone();
    view! {
        <div>
        <Card class="w-fit place-content-center">
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
<div class="snap-center flex-row">
  <h2 class="inline text-cyan-500 font-extrabold">Tags: </h2>
  <ul class="inline list-none p-0 m-0">
    {tags.into_iter().map(|tag| {
      view! {
        <li class="mr-2 inline">{tag}</li>
      }
    }).collect_view()}
  </ul>
        </div>
<div class="snap-center flex-row">
  <h2 class="inline text-cyan-500 font-extrabold">People: </h2>
  <ul class="inline list-none p-0 m-0">
    {people.into_iter().map(|person| {
      view! {
        <li class="mr-2 inline">{person}</li>
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


#[component]
pub fn VideoPlayer  (video_url: String) -> impl IntoView {
    let video_url=video_url.clone();
    let class_url = video_url.clone();

    view! {
    <div>
        <video controls width="600" height="400" src={class_url}>
            <source src={video_url} type="video/mp4" />
            Your browser does not support the video tag.
        </video>
    </div>
}
}
#[server]
pub async fn get_all_media_assets() -> Result<Vec<MediaWeb>, ServerFnError> {
    use crate::database::pg_calls::fetch_video_assets;
    let assets = fetch_video_assets().await;
    if let Ok(assets) = assets {
        Ok(assets)
    } else {
        Err(ServerFnError::new("Error fetching media assets"))
    }
}


