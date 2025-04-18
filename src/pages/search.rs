use crate::components::media_card::MediaCard;
use crate::components::shadcn_button::Button;
use crate::components::shadcn_card::{
    Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::lib_models::{MediaWeb, VideoMetadata};
use crate::pages::review::FallbackView;
use crate::settings::settings;
use leptos::attr::{controls, selected};
use leptos::either::Either;
use leptos::html::{video, Video};
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::Form;
use leptos_router::hooks::use_query_map;
use lucide_leptos::{BellRing, Check};
use wasm_bindgen::JsCast;

#[component]
pub fn SearchPage() -> impl IntoView {
    //get files
    //let files = Resource::new_blocking(
    //    || (),
    //    |_| async move {get_files().await.unwrap() },
    //);

    let selected_media = RwSignal::new(Vec::<i32>::new());
    let selected_media_count = RwSignal::new(0);
    let submit_search = ServerAction::<SearchMedia>::new();
    let search_results = submit_search.value();

    let on_selected_submit = move |_| {
        let selected_media = selected_media.clone().get();
        spawn_local(async {
            match process_selected_media(selected_media).await {
                Ok(_) => {
                    println!("Selected media processed successfully");
                    log!("Selected media processed successfully");
                    window()
                        .alert_with_message(&"Prepared!".to_string())
                        .unwrap();
                }
                Err(e) => {
                    window()
                        .alert_with_message(&format!("Error: {:?}", e))
                        .unwrap();

                    log!("Error processing selected media: {}", e);
                }
            }
        })
    };

    let files = Resource::new_blocking(
        || (),
        |_| async move { search_media("Tove".to_string()).await.unwrap() },
    );
    let fallback_message = &String::from("No files found");
    //hello world
    view! {
                <button on:click=on_selected_submit class="p-2 rounded-md bg-accent">
            "Prepare Selected: " {selected_media_count}
        </button>
    <div class="place-items-center text-primary">
        <div class="border-5 gap-10">
              <ActionForm action=submit_search>
        <label for="search">"Search: "</label>
        <input
          type="text"
          name="query"
            id="search"
          oninput="this.form.requestSubmit()"
        />
      </ActionForm>

        </div>
        <br/>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <div class="grid grid-cols-4 gap-4">
                    {move || {
                        search_results.get().map(|results| match results {
                            Ok(files) => {
                                files.iter().map(|f| {
                                    let media_id = f.id;
                                    let is_selected = Memo::new(move |_| {
                                        println!("something selected");
                                        selected_media.get().contains(&media_id)
                                    });

                                    // *** Corrected line ***
                                    let class_string = move || if is_selected.get() { "p-2 rounded-md bg-accent" } else { "p-2 rounded-md" };

                                    Either::Left(view! {
                                        <div on:click=move |_| {
                                            selected_media.update(|selected| {
                                                println!("clicked");
                                                if selected.contains(&media_id) {
                                                    *selected_media_count.write() += -1;
                                                    selected.retain(|&id| id != media_id);
                                                } else {
                                                    *selected_media_count.write() += 1;
                                                    selected.push(media_id);
                                                }
                                            });
                                        }
                                            class=class_string style="cursor: pointer;"> // Use the signal here
                                            <MediaCard media_web=f.clone() tags=None people=None editable=false/>
                                        </div>
                                    })
                                }).collect::<Vec<_>>()
                            },
                            Err(_) => vec![Either::Right(view!{<div></div>})],


                            // ... error handling
                        })
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

#[server(ProcessSelectedMedia)] // Give your server function a name
pub async fn process_selected_media(ids: Vec<i32>) -> Result<(), ServerFnError> {
    // Perform server-side actions with the selected IDs
    use crate::database::pg_calls::get_file_paths_by_ids;
    use crate::filesystem::fs_prepare::copy_files_to_destination;
    let settings = settings();
    let destination_path = &settings.processing.destination_path;
    log!("Server received IDs: {:?}", ids);
    //get filepaths
    let file_paths = get_file_paths_by_ids(ids);
    //copy files
    if let Ok(file_paths) = file_paths {
        let prepare_files = copy_files_to_destination(&file_paths, destination_path);
        if let Ok(prepare_files) = prepare_files {
            log!("Files prepared");
            Ok(())
        } else {
            Err(ServerFnError::new("Error preparing files"))
        }
    } else {
        Err(ServerFnError::new("Error fetching file paths"))
    }
}
