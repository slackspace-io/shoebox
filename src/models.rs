use crate::schema::media;
use crate::schema::media_people;
use crate::schema::media_tags;
use crate::schema::people;
use crate::schema::tags;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    pub id: i32,
    pub root_path: String,
    pub route: String,
    pub file_name: String,
    pub file_path: String,
    pub media_type: String,
    pub usable: Option<bool>,
    pub highlight: Option<bool>,
    pub reviewed: Option<bool>,
    pub description: Option<String>,
    pub duration_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub uploaded_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMedia {
    pub root_path: String,
    pub route: String,
    pub file_name: String,
    pub file_path: String,
    pub media_type: String,
    pub highlight: Option<bool>,
    pub usable: Option<bool>,
    pub reviewed: Option<bool>,
    pub duration_ms: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, AsChangeset)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MediaUpdate {
    pub file_name: String,
    pub usable: Option<bool>,
    pub highlight: Option<bool>,
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

#[derive(Queryable, Selectable, Debug, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(Media, foreign_key = media_id))]
#[diesel(belongs_to(Person, foreign_key = person_id))]
#[diesel(table_name = media_people)]
#[diesel(primary_key(media_id, person_id))]
pub struct MediaPerson {
    pub media_id: i32,
    pub person_id: i32,
}

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = people)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = people)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPerson<'a> {
    pub name: &'a str,
}
