use sqlx::{Pool, Postgres, Row};
use tracing::info;
use uuid::Uuid;

use crate::error::{AppError, Result};

pub struct LocationService {
    db: Pool<Postgres>,
}

impl LocationService {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    // Get all unique locations from videos
    pub async fn get_all_locations(&self) -> Result<Vec<String>> {
        let rows = sqlx::query(
            "SELECT DISTINCT location FROM videos
             WHERE location IS NOT NULL AND location != ''
             ORDER BY location"
        )
        .fetch_all(&self.db)
        .await
        .map_err(AppError::Database)?;

        let mut locations = Vec::new();
        for row in rows {
            let location: String = row.get("location");
            locations.push(location);
        }

        Ok(locations)
    }

    // Get location usage statistics
    pub async fn get_location_usage(&self) -> Result<Vec<LocationUsage>> {
        let rows = sqlx::query(
            "SELECT location, COUNT(*) as video_count
             FROM videos
             WHERE location IS NOT NULL AND location != ''
             GROUP BY location
             ORDER BY location"
        )
        .fetch_all(&self.db)
        .await
        .map_err(AppError::Database)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(LocationUsage {
                name: row.get("location"),
                video_count: row.get("video_count"),
            });
        }

        Ok(results)
    }

    // Update location across multiple videos
    pub async fn update_location(&self, old_location: &str, new_location: &str) -> Result<usize> {
        let result = sqlx::query(
            "UPDATE videos SET location = $1 WHERE location = $2"
        )
        .bind(new_location)
        .bind(old_location)
        .execute(&self.db)
        .await
        .map_err(AppError::Database)?;

        let count = result.rows_affected() as usize;
        if count > 0 {
            info!("Updated location '{}' to '{}' in {} videos", old_location, new_location, count);
        }

        Ok(count)
    }

    // Delete (set to NULL) location across multiple videos
    pub async fn delete_location(&self, location: &str) -> Result<usize> {
        let result = sqlx::query(
            "UPDATE videos SET location = NULL WHERE location = $1"
        )
        .bind(location)
        .execute(&self.db)
        .await
        .map_err(AppError::Database)?;

        let count = result.rows_affected() as usize;
        if count > 0 {
            info!("Removed location '{}' from {} videos", location, count);
        }

        Ok(count)
    }
}

// Location usage statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocationUsage {
    pub name: String,
    pub video_count: i64,
}
