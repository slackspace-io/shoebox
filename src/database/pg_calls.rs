use diesel::prelude::*;
use crate::models::{Media, MediaTag, MediaUpdate, MediaView, NewMedia, Tag};
use crate::schema::*;
use diesel::associations::HasTable;
use diesel::dsl::insert_into;
use diesel::query_dsl::methods::GroupByDsl;
use diesel::result::{DatabaseErrorKind, Error};
use leptos::prelude::ServerFnError;
use crate::database::pg_conn::pg_connection;
use crate::lib_models::MediaWeb;

pub fn return_all() -> QueryResult<usize> {
    let connection = &mut pg_connection();
    //left join media_tags on media.id = media_tags.media_id
    let media_view = media::table
        .left_outer_join(media_tags::table.on(media::id.eq(media_tags::media_id)))
        .left_outer_join(tags::table.on(media_tags::tag_id.eq(tags::id)))
        .select((Media::as_select(), Option::<(Tag)>::as_select()))
        .order(media::id)
        .limit(4)
        .load::<(Media, Option<Tag>)>(connection)
        .expect("Error loading media assets");
    println!("media_with_tags: {:?}", media_view);


    Ok(0)
}

//Working example..
pub fn associate_media_tags() -> QueryResult<usize> {
    let connection = &mut pg_connection();
    //let one_asset = media::table
    //    .select(Media::as_select())
    //    .get_results(connection)?;
    let all_assets=media::table
        .load::<Media>(connection)?;
    let all_tags = tags::table
        .load::<Tag>(connection)?;
    let media_mapping = MediaTag::belonging_to(&all_assets)
        .load::<MediaTag>(connection)?;
    let tag_mapping = MediaTag::belonging_to(&all_tags)
        .load::<MediaTag>(connection)?;
    let tags = all_tags.into_iter().zip(&tag_mapping).collect::<Vec<_>>();
    let media = all_assets.into_iter().zip(&media_mapping).collect::<Vec<_>>();
    let combo = media.into_iter().zip(tags).collect::<Vec<_>>();

    for row in combo {
        println!("media: {:?}", row);
    }
    Ok(0)
}

pub fn fetch_all_media_assets() -> Vec<Media> {
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


pub async fn fetch_video_assets() -> Result<Vec<MediaWeb>, ServerFnError> {
    use crate::models::{Media, NewMedia};
    //let result = return_all();
    let ass = associate_media_tags();

    //create MediaView struct

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





