use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Shoebox {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShoeboxDto {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShoeboxUsage {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub video_count: i64,
}

impl Shoebox {
    pub fn new(name: String, description: Option<String>) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
