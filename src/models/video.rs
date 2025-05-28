use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Video {
    pub id: String,
    pub file_path: String,
    pub file_name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created_date: Option<String>,
    pub file_size: Option<i64>,
    pub thumbnail_path: Option<String>,
    pub rating: Option<i32>,
    pub duration: Option<i64>,
    pub original_file_path: Option<String>,
    pub exif_data: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoWithMetadata {
    #[serde(flatten)]
    pub video: Video,
    pub tags: Vec<String>,
    pub people: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVideoDto {
    pub file_path: String,
    pub file_name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created_date: Option<String>,
    pub file_size: Option<i64>,
    pub thumbnail_path: Option<String>,
    pub rating: Option<i32>,
    pub duration: Option<i64>,
    pub original_file_path: Option<String>,
    pub exif_data: Option<serde_json::Value>,
    pub tags: Vec<String>,
    pub people: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVideoDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub rating: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub people: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSearchParams {
    pub query: Option<String>,
    pub tags: Option<Vec<String>>,
    pub people: Option<Vec<String>>,
    pub rating: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub unreviewed: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub video_ids: Vec<String>,
    pub project_name: String,
    #[serde(default)]
    pub use_original_files: bool,
}

impl Video {
    pub fn new(file_path: String, file_name: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            file_path,
            file_name,
            title: None,
            description: None,
            created_date: None,
            file_size: None,
            thumbnail_path: None,
            rating: None,
            duration: None,
            original_file_path: None,
            exif_data: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
