use sqlx::{Pool, Postgres, Row};
use tracing::info;
use uuid::Uuid;

use crate::error::{AppError, Result};

pub struct EventService {
    db: Pool<Postgres>,
}

impl EventService {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    // Get all unique events from videos
    pub async fn get_all_events(&self) -> Result<Vec<String>> {
        let rows = sqlx::query(
            "SELECT DISTINCT event FROM videos
             WHERE event IS NOT NULL AND event != ''
             ORDER BY event"
        )
        .fetch_all(&self.db)
        .await
        .map_err(AppError::Database)?;

        let mut events = Vec::new();
        for row in rows {
            let event: String = row.get("event");
            events.push(event);
        }

        Ok(events)
    }

    // Get event usage statistics
    pub async fn get_event_usage(&self) -> Result<Vec<EventUsage>> {
        let rows = sqlx::query(
            "SELECT event, COUNT(*) as video_count
             FROM videos
             WHERE event IS NOT NULL AND event != ''
             GROUP BY event
             ORDER BY event"
        )
        .fetch_all(&self.db)
        .await
        .map_err(AppError::Database)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(EventUsage {
                name: row.get("event"),
                video_count: row.get("video_count"),
            });
        }

        Ok(results)
    }

    // Update event across multiple videos
    pub async fn update_event(&self, old_event: &str, new_event: &str) -> Result<usize> {
        let result = sqlx::query(
            "UPDATE videos SET event = $1 WHERE event = $2"
        )
        .bind(new_event)
        .bind(old_event)
        .execute(&self.db)
        .await
        .map_err(AppError::Database)?;

        let count = result.rows_affected() as usize;
        if count > 0 {
            info!("Updated event '{}' to '{}' in {} videos", old_event, new_event, count);
        }

        Ok(count)
    }

    // Delete (set to NULL) event across multiple videos
    pub async fn delete_event(&self, event: &str) -> Result<usize> {
        let result = sqlx::query(
            "UPDATE videos SET event = NULL WHERE event = $1"
        )
        .bind(event)
        .execute(&self.db)
        .await
        .map_err(AppError::Database)?;

        let count = result.rows_affected() as usize;
        if count > 0 {
            info!("Removed event '{}' from {} videos", event, count);
        }

        Ok(count)
    }
}

// Event usage statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventUsage {
    pub name: String,
    pub video_count: i64,
}
