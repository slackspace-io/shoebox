use leptos::attr::controls;
use leptos::html::{video, Video};
use leptos::logging::log;
use leptos::prelude::*;
use lucide_leptos::{BellRing, Check};
use crate::components::shadcn_button::Button;
use crate::components::shadcn_card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::lib_models::{MediaWeb, VideoMetadata};
use crate::pages::review::MediaCard;

#[component]
pub fn BrowsePage() -> impl IntoView {
    //get files
    //let files = Resource::new_blocking(
    //    || (),
    //    |_| async move {get_files().await.unwrap() },
    //);
    let files = Resource::new_blocking(
        || (),
        |_| async move {get_all_media_assets().await.unwrap() },
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
                            <MediaCard media_web = f.clone() editable = false/>
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
pub async fn get_all_processed() -> Result<Vec<VideoMetadata>, ServerFnError> {
    use crate::database::return_all_video_assets;
    let processed = return_all_video_assets().expect("TODO: panic message");
    Ok(processed)
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




#[server]
//show directories and files of a given path
pub async fn get_files() -> Result<Vec<String>, ServerFnError> {
    let path = "/home/dopey/videos";
    let mut files = Vec::new();
    // Iterate over entries in the specified directory
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif") {
                    files.push(path.display().to_string());
                } else if matches!(ext.as_str(), "mp4" | "mkv" | "avi" | "mov") {
                    files.push(path.display().to_string());
                } else {
                    files.push(path.display().to_string());
                }
            }
        }
    }
    println!("{:?}", files);
    Ok(files)
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
pub fn CardDemo(media_web: MediaWeb) -> impl IntoView {
    let path = media_web.file_path.clone();
    let video_url = format!("/videos/{}", media_web.file_name);
    let file_name = media_web.file_name.clone();
    view! {
        <Card class="w-fit place-content-center">
            <CardHeader>
                <CardTitle>{file_name}</CardTitle>
                <CardDescription>"hi"</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-4">
                <div class=" flex items-center space-x-4 rounded-md border p-4">
                    <div class="flex-1 space-y-1">
                        <p class="text-sm font-medium leading-none">
                    <VideoPlayer video_url=video_url/>
                        </p>
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
    }
}


#[component]
pub fn VideoPlayer  (video_url: String) -> impl IntoView {
    view! {
    <div>
        <video controls width="600" height="400">
            <source src={video_url} type="video/mp4" />
            Your browser does not support the video tag.
        </video>
    </div>
}
}
