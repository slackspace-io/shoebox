use diesel::dsl::insert_into;
use crate::models::{Media, NewMedia};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

pub fn fetch_all_media_assets() -> Vec<Media> {
    use crate::database::pg_conn::pg_connection;
    use crate::schema::media::dsl::*;
    let connection = &mut pg_connection();
    let results = media
        .filter(reviewed.eq(false))
        .filter(media_type.eq("video"))
        .limit(5)
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


    //diesel::insert_into(media)
    //    .values(new_media)
    //    .get_result(connection)
    //    .expect("Error saving new media")
}
