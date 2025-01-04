use leptos::logging::log;
use leptos::prelude::Read;
use leptos::prelude::*;
use leptos_router::components::Form;
use leptos_router::hooks::use_query_map;
use crate::components::alert::AlertVariant::Default;
use crate::components::shadcn_button::{Button, ButtonVariant};
use crate::components::shadcn_input::{Input, InputType};
use crate::lib_models::{MediaWeb, Metadata, VideoMetadata};

#[server]
async fn handle_tags(tags: String) -> Result<Vec<i32>, ServerFnError> {
    use crate::database::pg_inserts::insert_new_tag;
    use crate::models::NewTag;
    let mut tag_ids = Vec::new();
    log!("Handling tags");
    //split tags by comma
    let tags = tags.split(",").collect::<Vec<&str>>();
    log!("Tags: {:?}", tags);
    for tag in tags {
        //insert new tag into db
        let new_tag = NewTag {
            name: tag,
        };
        let tag_id = insert_new_tag(&new_tag);
        tag_ids.push(tag_id?);
        //check if tag exists
        //if tag does not exist, create tag
        //if tag exists, get tag id
    }
    log!("Tag ids: {:?}", tag_ids);
    Ok(tag_ids)

}

#[server]
async fn handle_people(people: String) -> Result<Vec<i32>, ServerFnError> {
    use crate::database::pg_inserts::insert_new_person;
    use crate::models::NewPerson;
    let mut person_ids = Vec::new();
    log!("Handling tags");
    //split tags by comma
    let people_list = people.split(",").collect::<Vec<&str>>();
    log!("Tags: {:?}", people_list);
    for person in people_list {
        //insert new tag into db
        let new_person = NewPerson {
            name: person,
        };
        let person_id = insert_new_person(&new_person);
        person_ids.push(person_id?);
        //check if tag exists
        //if tag does not exist, create tag
        //if tag exists, get tag id
    }
    log!("Tag ids: {:?}", person_ids);
    Ok(person_ids)

}




#[server]
async fn handle_form(tags: String, people: String, good_take: String, file: String, description: String) -> Result<(), ServerFnError> {
    use crate::database::pg_updates::update_media;
    use crate::database::pg_inserts::insert_new_media_person;
    use crate::database::pg_inserts::insert_new_media_tag;
    use crate::models::MediaUpdate;
    use crate::models::MediaTag;
    use crate::models::MediaPerson;
    log!("File within handle_form: {:?}", file);
    log!("Handling form");
    let tag_ids = match handle_tags(tags).await {
        Ok(tag_ids) => tag_ids,
        Err(e) => {
            log!("Error handling tags: {:?}", e);
            return Err(e);
        }
    };
    log!("Tag ids: {:?}", tag_ids);
    let person_ids = match handle_people(people).await {
        Ok(person_ids) => person_ids,
        Err(e) => {
            log!("Error handling people: {:?}", e);
            return Err(e);
        }
    };
    let media_update = MediaUpdate {
        file_name: file,
        reviewed: Some(true),
        description,
    };


    //update db
    let media_update_results = update_media(&media_update);
    let media_id = match media_update_results {
        Ok(media_id) => {
            log!("Media id: {:?}", media_id);
            media_id
        }
        Err(e) => {
            log!("Error updating media: {:?}", e);
            return Err(ServerFnError::from(e));
        }
    };
    log!("Media id: {:?}", media_id);
    log!("Updated video metadata");
    for tag_id in tag_ids {
        let media_tag = MediaTag {
            media_id,
            tag_id,
        };
        //insert media tag
        insert_new_media_tag(media_tag);
    }
    for person_id in person_ids {
        let media_person = MediaPerson {
            media_id,
            person_id,
        };
        //insert media tag
        insert_new_media_person(media_person);
    }
    //update media tags
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
