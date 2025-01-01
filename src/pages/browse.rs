use leptos::html::video;
use leptos::logging::log;
use leptos::prelude::*;
use lucide_leptos::{BellRing, Check};
use crate::components::shadcn_button::Button;
use crate::components::shadcn_card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::lib_models::VideoMetadata;

#[component]
pub fn BrowsePage() -> impl IntoView {
    //get files
    //let files = Resource::new_blocking(
    //    || (),
    //    |_| async move {get_files().await.unwrap() },
    //);
    let files = Resource::new_blocking(
        || (),
        |_| async move {get_all_processed().await.unwrap() },
    );
    let fallback_message = &String::from("No files found");
//hello world
    view! {
    <Suspense
    fallback= move || {
        view! {
            <p>"Loading..."</p>
        }
    }>
    //list files
    <div>
        {move || files.get().iter().next().map(|file| {
            view! {
                <div>
                    {file.iter().map(|f| {
                        view! {
                            <CardDemo video_metadata = f.clone()/>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            }
        })}
    </div>

    </Suspense>
    }
}


#[server]
pub async fn get_all_processed() -> Result<Vec<VideoMetadata>, ServerFnError> {
    use crate::database::return_all_video_assets;
    let processed = return_all_video_assets().expect("TODO: panic message");
    Ok(processed)
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
pub fn CardDemo(video_metadata: VideoMetadata) -> impl IntoView {
    let path = video_metadata.metadata.path.clone();
    let video_url = video_metadata.video_url();
    let good_take = video_metadata.metadata.good_take.clone();
    let yearly_highlight = video_metadata.metadata.yearly_highlight.clone();
    let people = video_metadata.metadata.people.clone();
    view! {
        <Card class="w-fit">
            <CardHeader>
                <CardTitle>{path}</CardTitle>
                <CardDescription>{good_take}</CardDescription>
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
                                <p>{format!("{:?}", video_url)}</p>
                                <video controls width="600"
                                src={video_url}
                            >
                                    "Your browser does not support the video tag."
                                </video>
                            </div>
                        }

}
