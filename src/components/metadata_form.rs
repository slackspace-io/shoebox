use leptos::logging::log;
use leptos::prelude::Read;
use leptos::prelude::*;
use leptos_router::components::Form;
use leptos_router::hooks::use_query_map;
use crate::components::alert::AlertVariant::Default;
use crate::components::shadcn_button::{Button, ButtonVariant};
use crate::components::shadcn_input::{Input, InputType};
use crate::lib_models::{MediaWeb, Metadata, VideoMetadata};

async fn handle_form_results(metadata_results: VideoMetadata) {
    log!("Handling form results");

}

#[server]
async fn handle_form(tags: String, people: String, good_take: String, file: String, description: String) -> Result<(), ServerFnError> {
    use crate::database::update_video_metadata;
    use crate::models::MediaUpdate;
    log!("File within handle_form: {:?}", file);
    log!("Handling form");
    let media_update = MediaUpdate {
        id: 1,
        reviewed: Some(true),
        description,
    };

    //update db
   // update_video_metadata(metadata_results).expect("TODO: panic message");
    log!("Updated video metadata");
    //redirect to homepage
    //reload home page
   // leptos_axum::redirect("/review/next");

    Ok(())
}

//Make form for video_metadatal model
#[component]
pub fn VideoMetadataForm(file: String) -> impl IntoView {
    let submit = ServerAction::<HandleForm>::new();
    //handle form data after submit
    view! {
        <div class="form">
        <ActionForm action=submit >
        <div>
        <label for="description">"Description: "</label>
        <Input r#type=InputType::Text id="description" name="description" />
        </div>
        <div>
            <label for="people">"People: "</label>
            <Input r#type=InputType::Text id="people" name="people" />
        </div>
        <div>
        <label for="tags">"Tags: "</label>
        <Input r#type=InputType::Text id="tags" name="tags"  />
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
            <Button r#type="Submit" >"Submit"</Button>
        </div>
        </ActionForm>
        </div>

    }
}
