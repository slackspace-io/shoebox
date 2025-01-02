use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::schema::media;


#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    pub id: i32,
    pub file_path: String,
    pub media_type: String,
    pub reviewed: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub uploaded_at: Option<DateTime<Utc>>
}

#[derive(Insertable, Debug)]
#[diesel(table_name = media)]
pub struct NewMedia {
    pub file_path: String,
    pub media_type: String,
    pub reviewed: Option<bool>,
    pub created_at: DateTime<Utc>,
}
