use leptos::logging::log;
use leptos::prelude::Read;
use leptos::prelude::*;
use leptos_router::components::Form;
use leptos_router::hooks::use_query_map;
use crate::components::shadcn_button::{Button, ButtonVariant};
use crate::components::shadcn_input::{Input, InputType};
use crate::lib_models::{Metadata, VideoMetadata};

async fn handle_form_results(metadata_results: VideoMetadata) {
    log!("Handling form results");

}

#[server]
async fn handle_form(pets: String, people: String, good_take: String, file: String) -> Result<(), ServerFnError> {
    use crate::database::update_video_metadata;
    log!("File within handle_form: {:?}", file);
    log!("Handling form");
    let metadata_results = VideoMetadata {
        path: file.clone(),
        metadata: Metadata {
            asset_type: "video".to_string(),
            path: file.clone(),
            file_name: "test".to_string(),
            creation_date: "".to_string(),
            good_take: good_take.parse().unwrap(),
            yearly_highlight: "true".parse().unwrap(),
            people: people.parse().unwrap(),
            pets: pets.parse().unwrap(),
            location: "test".to_string(),
            processed: "true".to_string(),
            discovery_date: "".to_string(),

        },
    };
    log!("Metadata results: {:?}", metadata_results);
    //update db
    update_video_metadata(metadata_results).expect("TODO: panic message");
    log!("Updated video metadata");
    //redirect to homepage
    //reload home page
    leptos_axum::redirect("/review/next");

    Ok(())
}

//Make form for video_metadatal model
#[component]
pub fn VideoMetadataForm(file: String) -> impl IntoView {
    let submit = ServerAction::<HandleForm>::new();
    //handle form data after submit
    view! {
        <div class="form">
        <h1>"Video Metadata Form"</h1>
        <ActionForm action=submit >
        <div>
            <label for="people">"People: "</label>
            <Input r#type=InputType::Text id="people" name="people" />
        </div>
        <div>
        <label for="pets">"Pets: "</label>
        <Input r#type=InputType::Text id="pets" name="pets"  />
        </div>
        <div class="good_take">
        <fieldset>
            <legend>"Good Take"</legend>
            <div>
                <input type="radio" id="good_take" name="good_take" value="true" />
                <label for="good_take">"True"</label>
            </div>
            <div>
                <input type="radio" id="good_take" name="good_take" value="false" />
                <label for="good_take">"False"</label>
            </div>
        </fieldset>
        </div>
        <div>
            <input type="hidden" name="file" value=file   />
            <Button r#type="Submit" variant=ButtonVariant::Secondary>"Submit"</Button>
        </div>
        </ActionForm>
        </div>

    }
}
