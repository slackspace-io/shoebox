use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum FileType {
    Video(String),
    Photo(String),
    Other(String),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct VideoMetadata {
    pub path: String,
    //include metadata struct
    pub metadata: Metadata,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Metadata {
    pub good_take: String,
    pub yearly_highlight: String,
    pub people: String,
    pub pets: String,
    //optional fields
    pub location: String,
    pub processed: String,
    pub asset_type: String,
    pub path: String,
    pub file_name: String,
    pub creation_date: String,
    pub discovery_date: String,
}

impl VideoMetadata {
    pub fn video_url(&self) -> String {
        format!("/videos/{}", self.metadata.file_name)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MediaFile {
    pub asset_type: String,
    pub path: String,
    pub file_name: String,
    pub creation_date: String,
    pub discovery_date: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MediaWeb {
    pub id: i32,
    pub file_path: String,
    pub file_name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub people: Vec<String>,
    pub media_type: String,
    pub highlight: Option<bool>,
    pub good_take: Option<bool>,
    pub reviewed: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub uploaded_at: Option<DateTime<Utc>>,
}

impl MediaWeb {
    pub fn file_name_no_ext(&self) -> String {
        self.file_name.split('.').next().unwrap().to_string()
    }
}
