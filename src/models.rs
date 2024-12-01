use diesel::prelude::*;
use crate::schema::media_files;
use diesel::sql_types::{Integer, Nullable, Text, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::media_files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct MediaFile {
    id: Option<i32>,
    asset_type: String,
    path: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = media_files)]
pub struct NewMediaFile<'a> {
    pub asset_type: &'a str,
    pub path: &'a str,
}

