use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::schema::media;
use crate::schema::tags;
use crate::schema::media_tags;


#[derive(Queryable, Selectable, Debug, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    pub id: i32,
    pub file_path: String,
    pub file_name: String,
    pub media_type: String,
    pub reviewed: Option<bool>,
    pub description: Option<String>,
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

#[derive(Queryable, Selectable, Debug, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(Media, foreign_key = media_id))]
#[diesel(belongs_to(Tag, foreign_key = tag_id))]
#[diesel(table_name = media_tags)]
#[diesel(primary_key(media_id, tag_id))]
pub struct MediaTag {
    pub media_id: i32,
    pub tag_id: i32,
}


#[derive(Queryable, Selectable, Debug, Serialize, Deserialize, Identifiable)]
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


#[derive(Debug, Serialize, Deserialize )]
pub struct MediaView {
    pub media: Media,
    pub tags: Vec<Tag>,
}


