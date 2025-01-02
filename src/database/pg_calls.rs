use diesel::dsl::insert_into;
use crate::models::{Media, MediaUpdate, NewMedia};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use leptos::prelude::ServerFnError;
use crate::lib_models::MediaWeb;

pub fn fetch_all_media_assets() -> Vec<Media> {
    use crate::database::pg_conn::pg_connection;
    use crate::schema::media::dsl::*;
    let connection = &mut pg_connection();
    let results = media
        .filter(media_type.eq("video"))
        .limit(10)
        .select(Media::as_select())
        .load(connection)
        .expect("Error loading media assets");
    results
}

pub fn insert_new_media(new_media: &NewMedia) -> QueryResult<usize>{
    use crate::database::pg_conn::pg_connection;
    use crate::schema::media::dsl::*;
    let connection = &mut pg_connection();
    let result = insert_into(media)
        .values(new_media)
        .execute(connection);
    if let Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) = result {
        println!("Media already exists in pgsql");
        Ok(0)
    } else {
        println!("Media inserted");
        result

    }
}


pub fn update_media(media_update: &MediaUpdate) -> QueryResult<usize> {
    use crate::database::pg_conn::pg_connection;
    use crate::schema::media::dsl::*;
    let connection = &mut pg_connection();
    let result = diesel::update(media)
        .filter(file_name.eq(&media_update.file_name))
        .set(media_update)
        .execute(connection);
    result
}

pub async fn fetch_video_assets() -> Result<Vec<MediaWeb>, ServerFnError> {
    use crate::models::{Media, NewMedia};

    let assets = fetch_all_media_assets();
    let web_assets = assets.iter().map(|asset| {
        MediaWeb {
            id: asset.id,
            file_path: asset.file_path.clone(),
            file_name: asset.file_name.clone(),
            media_type: asset.media_type.clone(),
            reviewed: asset.reviewed,
            created_at: asset.created_at,
            uploaded_at: asset.uploaded_at,
            description: asset.description.clone(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            people: vec!["person1".to_string(), "person2".to_string()],


        }
    }).collect();
    Ok(web_assets)

}



