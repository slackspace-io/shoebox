use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonDto {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonUsage {
    pub id: String,
    pub name: String,
    pub video_count: i64,
}

impl Person {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}
