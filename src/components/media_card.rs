use leptos::either::Either;
use leptos::prelude::*;
use crate::components::metadata_form::VideoMetadataForm;
use crate::components::shadcn_card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::lib_models::MediaWeb;

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
