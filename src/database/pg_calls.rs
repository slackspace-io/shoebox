use crate::models::{Media, NewMedia};
use diesel::prelude::*;

pub fn fetch_all_media_assets() -> Vec<Media> {
    use crate::database::pg_conn::pg_connection;
    use crate::schema::media::dsl::*;
    let connection = &mut pg_connection();
    let results = media
        .filter(reviewed.eq(false))
        .limit(5)
        .select(Media::as_select())
        .load(connection)
        .expect("Error loading media assets");
    results
}

pub fn insert_new_media(new_media: &NewMedia) -> Media{
    use crate::database::pg_conn::pg_connection;
    use crate::schema::media::dsl::*;
    let connection = &mut pg_connection();
    diesel::insert_into(media)
        .values(new_media)
        .get_result(connection)
        .expect("Error saving new media")
}

