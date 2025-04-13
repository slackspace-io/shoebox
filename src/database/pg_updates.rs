use crate::models::MediaUpdate;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::{QueryResult, RunQueryDsl};

pub fn update_media(media_update: &MediaUpdate) -> QueryResult<i32> {
    use crate::database::pg_conn::pg_connection;
    use crate::schema::media::dsl::*;
    let connection = &mut pg_connection();
    let result = diesel::update(media)
        .filter(file_name.eq(&media_update.file_name))
        .set(media_update)
        .returning(id)
        .get_result(connection);
    result
}
