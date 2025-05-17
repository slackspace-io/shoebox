// src/components/remove_handlers.rs
use leptos::logging::log;
use leptos::server;
use leptos::server_fn::ServerFnError;

#[server]
pub async fn remove_person(person: String, media_id: i32) -> Result<(), ServerFnError> {
    use crate::database::pg_deletes;
    log!("Removing person: {}", person);
    log!("From media_id: {}", media_id);
    pg_deletes::remove_person(media_id, person).await?;
    Ok(())
}

#[server]
pub async fn remove_tag(tag: String, media_id: i32) -> Result<(), ServerFnError> {
    use crate::database::pg_deletes;
    log!("Removing tag: {}", tag);
    log!("From media_id: {}", media_id);
    pg_deletes::remove_tag(media_id, tag).await?;
    Ok(())
}
