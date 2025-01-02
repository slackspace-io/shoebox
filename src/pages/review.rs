use leptos::attr::formaction;
use leptos::either::Either;
use leptos::ev::MouseEvent;
use leptos::html::{video};
use leptos::logging::log;
use leptos::prelude::*;
use lucide_leptos::{BellRing, Check};
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
                        <MediaCard media_web={f.clone()} />
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
                        <MediaCard media_web={f.clone()} />
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
pub fn MediaCard(media_web: MediaWeb) -> impl IntoView {
    let path = media_web.file_path.clone();
    let video_url = format!("/videos/{}", media_web.file_name);
    let file_name = media_web.file_name.clone();
    let description = media_web.description.clone();
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
                <CardTitle>{description}</CardTitle>
                <CardDescription>"hi"</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-4">
                <div class=" flex items-center space-x-4 rounded-md border p-4">
                    <div class="flex-1 space-y-1">
                    <VideoPlayer video_url=video_url/>
                        <p class="text-sm text-muted-foreground ">
                            {"Send notifications to device."}
                        </p>
                    </div>

                </div>
                <div>
                    <For
                        each=move || notifications()
                        key=|notification| notification.id
                        children=move |notification: Notification| {
                    view! {
                        <div
                            class="mb-4 grid grid-cols-[25px_1fr] items-start pb-4 last:mb-0 last:pb-0"
                        >
                            <span class="flex h-2 w-2 translate-y-1 rounded-full bg-sky-500" />
                            <div class="space-y-1">
                                <p class="text-sm font-medium leading-none">
                                    {notification.title}
                                </p>
                                <p class="text-sm text-muted-foreground">
                                    {notification.description}
                                </p>
                            </div>
                        </div>
                    }
                }
                    />
                </div>
            </CardContent>
            <CardFooter>
                <Button class="w-full">
                    <Check />{" Mark all as read"}
                </Button>
            </CardFooter>
        </Card>
        </div>
    }
}


#[component]
pub fn VideoPlayer  (video_url: String) -> impl IntoView {
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


