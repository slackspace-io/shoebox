use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::components::alert::AlertDescriptionProps;
use crate::schema::media;
use crate::schema::media::description;
use crate::schema::tags;
use crate::schema::media_tags;


#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    pub id: i32,
    pub file_path: String,
    pub file_name: String,
    pub media_type: String,
    pub description: Option<String>,
    pub reviewed: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub uploaded_at: Option<DateTime<Utc>>
}

#[derive(Insertable, Debug)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMedia {
    pub file_path: String,
    pub file_name: String,
    pub media_type: String,
    pub reviewed: Option<bool>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, AsChangeset)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MediaUpdate {
    pub file_name: String,
    pub reviewed: Option<bool>,
    pub description: String,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = media_tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MediaTag {
    pub media_id: i32,
    pub tag_id: i32,
}


#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTag<'a> {
    pub name: &'a str,
}
