use crate::components::media_card::MediaCard;
use crate::components::shadcn_button::Button;
use crate::components::shadcn_card::{
    Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::lib_models::{MediaWeb, VideoMetadata};
use crate::pages::review::FallbackView;
use leptos::attr::controls;
use leptos::either::Either;
use leptos::html::{video, Video};
use leptos::logging::log;
use leptos::prelude::*;
use leptos_router::components::Form;
use leptos_router::hooks::use_query_map;
use lucide_leptos::{BellRing, Check};

#[component]
pub fn SearchPage() -> impl IntoView {
    //get files
    //let files = Resource::new_blocking(
    //    || (),
    //    |_| async move {get_files().await.unwrap() },
    //);
    let submit_search = ServerAction::<SearchMedia>::new();
    let search_results = submit_search.value();

    let files = Resource::new_blocking(
        || (),
        |_| async move { search_media("Tove".to_string()).await.unwrap() },
    );
    let fallback_message = &String::from("No files found");
    //hello world
    view! {
    <div class="place-items-center">
              <ActionForm action=submit_search>
        <input
          type="text"
          name="query"
          oninput="this.form.requestSubmit()"
        />
      </ActionForm>

            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <div class="grid grid-cols-4 gap-4">
                    {move || {
                        search_results.get().map(|results| match results {
                            Ok(files) => {
                                files.iter().map(|f| {
                                    Either::Right(view! { <MediaCard media_web=f.clone() editable=false/> })
                                }).collect::<Vec<_>>()

                            }
                            Err(e) => {
                                vec![Either::Left(view!{ <div></div>})]
                            }
                        }).unwrap_or_default()
                    }}
                </div>
            </Suspense>
    </div>
    }
}

#[server]
pub async fn search_media(query: String) -> Result<Vec<MediaWeb>, ServerFnError> {
    use crate::database::pg_calls::fetch_video_assets;
    println!("Searching for: {} ", query);
    use crate::database::pg_calls::search_media_assets;
    let assets = search_media_assets(query.as_str()).await;

    // let assets = fetch_video_assets(false).await;
    if let Ok(assets) = assets {
        println!("assets: {:?} ", assets);
        Ok(assets)
    } else {
        Err(ServerFnError::new("Error fetching media assets"))
    }
}
