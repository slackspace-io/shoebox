use leptos::logging::log;
use leptos::prelude::Read;
use leptos::prelude::*;
use leptos_router::components::Form;
use leptos_router::hooks::use_query_map;
use crate::lib_models::{Metadata, VideoMetadata};
use crate::models::MediaFile;

#[component]
pub fn FormExample() -> impl IntoView {
    // reactive access to URL query
    let query = use_query_map();
    let name = move || query.read().get("name").unwrap_or_default();
    let number = move || query.read().get("number").unwrap_or_default();
    let select = move || query.read().get("select").unwrap_or_default();

    view! {
        // read out the URL query strings
        <table>
            <tr>
                <td><code>"name"</code></td>
                <td>{name}</td>
            </tr>
            <tr>
                <td><code>"number"</code></td>
                <td>{number}</td>
            </tr>
            <tr>
                <td><code>"select"</code></td>
                <td>{select}</td>
            </tr>
        </table>
        // <Form/> will navigate whenever submitted
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="">
            // input names determine query string key
            <input type="text" name="name" value=name/>
            <input type="number" name="number" value=number/>
            <select name="select">
                // `selected` will set which starts as selected
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
        // This <Form/> uses some JavaScript to submit
        // on every input

    }
}

async fn handle_form_results(metadata_results: VideoMetadata) {
    log!("Handling form results");

}

#[server]
async fn handle_form(pets: String, people: String, good_take: String, file: String) -> Result<(), ServerFnError> {
    use crate::database::update_video_metadata;
    log!("File within handle_form: {:?}", file);
    log!("Handling form");
    let metadata_results = VideoMetadata {
        path: file,
        metadata: Metadata {
            good_take: good_take.parse().unwrap(),
            yearly_highlight: "true".parse().unwrap(),
            people: people.parse().unwrap(),
            pets: pets.parse().unwrap(),
            location: "test".to_string(),
            processed: "true".to_string(),
        },
    };
    log!("Metadata results: {:?}", metadata_results);
    //update db
    update_video_metadata(metadata_results).expect("TODO: panic message");
    log!("Updated video metadata");
    Ok(())
}

//Make form for video_metadatal model
#[component]
pub fn VideoMetadataForm(file: String) -> impl IntoView {
    let submit = ServerAction::<HandleForm>::new();
    ///let query = use_query_map();
   ///// let name = move || query.read().get("name").unwrap_or_default();
   ///// let number = move || query.read().get("number").unwrap_or_default();
   ///// let select = move || query.read().get("select").unwrap_or_default();
    ///let good_take = move || query.read().get("good_take").unwrap_or_default();
    ///let yearly_highlight = move || query.read().get("yearly_highlight").unwrap_or_default();
    ///let people = move || query.read().get("people").unwrap_or_default();
    ///let pets = move || query.read().get("pets").unwrap_or_default();
    ///let location = move || query.read().get("location").unwrap_or_default();
    ///let processed = move || query.read().get("processed").unwrap_or_default();
    ///let metadata_results = VideoMetadata {
    ///    path: "".to_string(),
    ///    metadata: Metadata {
    ///        good_take: query.read().get("good_take").unwrap_or_default().parse().unwrap(),
    ///        yearly_highlight: query.read().get("yearly_highlight").unwrap_or_default().parse().unwrap(),
    ///        people: Option::from(query.read().get("people").unwrap_or_default()),
    ///        pets: Option::from(query.read().get("pets").unwrap_or_default()),
    ///        location: Option::from(query.read().get("location").unwrap_or_default()),
    ///        processed: true,
    ///    },
    ///};
    //handle results after form submitted

    //handle form data after submit
    view! {
        <h1>"Video Metadata Form"</h1>
        <ActionForm action=submit >
            <input type="text" name="people" />
        <p>People</p>
            <input type="text" name="pets" />
        <p>Pets</p>
            <select name="good_take">
                <option>
                    "True"
                </option>
                <option>
                    "False"
                </option>
            </select>
        <p>Good Take</p>
            <input type="hidden" name="file" value=file   />
            <input type="submit" />
        </ActionForm>

    }
}
