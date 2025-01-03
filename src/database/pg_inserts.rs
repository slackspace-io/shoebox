use diesel::dsl::insert_into;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::query_dsl::QueryDsl;
use diesel::prelude::*;


use diesel::{QueryResult, RunQueryDsl};
use crate::models::{MediaTag, NewTag, Tag};
use crate::database::pg_conn::pg_connection;

pub fn insert_new_tag(new_tag: &NewTag) -> QueryResult<i32> {
    use crate::schema::tags::dsl::*;
    let connection = &mut pg_connection();

    match insert_into(tags)
        .values(new_tag)
        .on_conflict(name)
        .do_nothing()
        .returning(id)
        .get_result(connection)
    {
        Ok(tag_id) => Ok(tag_id), // Successfully inserted, return the id.
        Err(diesel::result::Error::NotFound) => {
            // Conflict occurred, fetch the id of the existing tag.
            tags.filter(name.eq(&new_tag.name)) // Pass the name as a reference directly.
                .select(id)
                .first(connection)
        }
        Err(e) => Err(e), // Propagate other errors.
    }
}


pub fn insert_new_media_tag(new_media_tag: MediaTag) -> QueryResult<usize> {
    use crate::schema::media_tags::dsl::*;
    let connection = &mut pg_connection();
    insert_into(media_tags)
        .values(new_media_tag)
        .execute(connection)
}
