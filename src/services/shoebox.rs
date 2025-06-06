use sqlx::{Pool, Postgres, Transaction, Row};
use tracing::{info, warn};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{Shoebox, CreateShoeboxDto, ShoeboxUsage};

pub struct ShoeboxService {
    db: Pool<Postgres>,
}

impl ShoeboxService {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Shoebox>> {
        let shoeboxes = sqlx::query_as::<_, Shoebox>("SELECT * FROM shoeboxes ORDER BY name")
            .fetch_all(&self.db)
            .await
            .map_err(AppError::Database)?;

        Ok(shoeboxes)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Shoebox> {
        let shoebox = sqlx::query_as::<_, Shoebox>("SELECT * FROM shoeboxes WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound(format!("Shoebox not found: {id}")),
                _ => AppError::Database(e),
            })?;

        Ok(shoebox)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Shoebox> {
        let shoebox = sqlx::query_as::<_, Shoebox>("SELECT * FROM shoeboxes WHERE name = $1")
            .bind(name)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    AppError::NotFound(format!("Shoebox not found: {name}"))
                }
                _ => AppError::Database(e),
            })?;

        Ok(shoebox)
    }

    pub async fn find_or_create_by_name(
        &self,
        name: &str,
        description: Option<&str>,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<String> {
        // Try to find existing shoebox
        let shoebox_result = sqlx::query_as::<_, Shoebox>("SELECT * FROM shoeboxes WHERE name = $1")
            .bind(name)
            .fetch_optional(&mut **tx)
            .await
            .map_err(AppError::Database)?;

        if let Some(shoebox) = shoebox_result {
            return Ok(shoebox.id);
        }

        // Create new shoebox
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().naive_utc();

        sqlx::query("INSERT INTO shoeboxes (id, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)")
            .bind(&id)
            .bind(name)
            .bind(description)
            .bind(&now)
            .bind(&now)
            .execute(&mut **tx)
            .await
            .map_err(AppError::Database)?;

        info!("Created new shoebox: {name} ({id})");
        Ok(id)
    }

    pub async fn create(&self, dto: CreateShoeboxDto) -> Result<Shoebox> {
        // Check if shoebox already exists
        let existing = sqlx::query_as::<_, Shoebox>("SELECT * FROM shoeboxes WHERE name = $1")
            .bind(&dto.name)
            .fetch_optional(&self.db)
            .await
            .map_err(AppError::Database)?;

        if let Some(shoebox) = existing {
            return Ok(shoebox);
        }

        let shoebox = Shoebox::new(dto.name, dto.description);

        sqlx::query("INSERT INTO shoeboxes (id, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)")
            .bind(&shoebox.id)
            .bind(&shoebox.name)
            .bind(&shoebox.description)
            .bind(&shoebox.created_at)
            .bind(&shoebox.updated_at)
            .execute(&self.db)
            .await
            .map_err(AppError::Database)?;

        info!("Created new shoebox: {0} ({1})", shoebox.name, shoebox.id);
        Ok(shoebox)
    }

    pub async fn update(&self, id: &str, name: &str, description: Option<&str>) -> Result<Shoebox> {
        // Check if shoebox exists
        let shoebox = self.find_by_id(id).await?;

        // Check if the new name already exists
        let existing = sqlx::query_as::<_, Shoebox>("SELECT * FROM shoeboxes WHERE name = $1 AND id != $2")
            .bind(name)
            .bind(id)
            .fetch_optional(&self.db)
            .await
            .map_err(AppError::Database)?;

        if existing.is_some() {
            return Err(AppError::BadRequest(format!(
                "Shoebox with name '{name}' already exists"
            )));
        }

        // Update shoebox
        let now = chrono::Utc::now().naive_utc();
        sqlx::query("UPDATE shoeboxes SET name = $1, description = $2, updated_at = $3 WHERE id = $4")
            .bind(name)
            .bind(description)
            .bind(&now)
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(AppError::Database)?;

        info!("Updated shoebox: {0} -> {name} ({id})", shoebox.name);

        // Return updated shoebox
        let updated_shoebox = self.find_by_id(id).await?;
        Ok(updated_shoebox)
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        // Check if shoebox exists
        let shoebox = self.find_by_id(id).await?;

        // Check if shoebox is in use
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM video_shoeboxes WHERE shoebox_id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await
            .map_err(AppError::Database)?;

        if count > 0 {
            return Err(AppError::BadRequest(format!(
                "Cannot delete shoebox '{}' because it contains {} videos",
                shoebox.name, count
            )));
        }

        // Delete shoebox
        sqlx::query("DELETE FROM shoeboxes WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(AppError::Database)?;

        info!("Deleted shoebox: {} ({})", shoebox.name, id);
        Ok(())
    }

    pub async fn get_usage(&self) -> Result<Vec<ShoeboxUsage>> {
        let rows = sqlx::query(
            "SELECT s.id, s.name, s.description, COUNT(vs.video_id) as video_count
             FROM shoeboxes s
             LEFT JOIN video_shoeboxes vs ON s.id = vs.shoebox_id
             GROUP BY s.id
             ORDER BY s.name",
        )
        .fetch_all(&self.db)
        .await
        .map_err(AppError::Database)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(ShoeboxUsage {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                video_count: row.get("video_count"),
            });
        }

        Ok(results)
    }

    pub async fn cleanup_unused(&self) -> Result<usize> {
        let result = sqlx::query(
            "DELETE FROM shoeboxes
             WHERE id NOT IN (SELECT DISTINCT shoebox_id FROM video_shoeboxes)",
        )
        .execute(&self.db)
        .await
        .map_err(AppError::Database)?;

        let count = result.rows_affected() as usize;
        if count > 0 {
            info!("Cleaned up {} unused shoeboxes", count);
        }

        Ok(count)
    }

    pub async fn add_video_to_shoebox(&self, video_id: &str, shoebox_id: &str) -> Result<()> {
        // Check if the relationship already exists
        let exists = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM video_shoeboxes WHERE video_id = $1 AND shoebox_id = $2",
        )
        .bind(video_id)
        .bind(shoebox_id)
        .fetch_one(&self.db)
        .await
        .map_err(AppError::Database)?;

        if exists > 0 {
            return Ok(());
        }

        // Add the relationship
        sqlx::query(
            "INSERT INTO video_shoeboxes (video_id, shoebox_id, created_at) VALUES ($1, $2, $3)",
        )
        .bind(video_id)
        .bind(shoebox_id)
        .bind(chrono::Utc::now().naive_utc())
        .execute(&self.db)
        .await
        .map_err(AppError::Database)?;

        info!("Added video {video_id} to shoebox {shoebox_id}");
        Ok(())
    }

    pub async fn remove_video_from_shoebox(&self, video_id: &str, shoebox_id: &str) -> Result<()> {
        sqlx::query(
            "DELETE FROM video_shoeboxes WHERE video_id = $1 AND shoebox_id = $2",
        )
        .bind(video_id)
        .bind(shoebox_id)
        .execute(&self.db)
        .await
        .map_err(AppError::Database)?;

        info!("Removed video {video_id} from shoebox {shoebox_id}");
        Ok(())
    }

    pub async fn get_videos_in_shoebox(&self, shoebox_id: &str) -> Result<Vec<String>> {
        let video_ids = sqlx::query_scalar::<_, String>(
            "SELECT video_id FROM video_shoeboxes WHERE shoebox_id = $1",
        )
        .bind(shoebox_id)
        .fetch_all(&self.db)
        .await
        .map_err(AppError::Database)?;

        Ok(video_ids)
    }

    pub async fn get_shoeboxes_for_video(&self, video_id: &str) -> Result<Vec<Shoebox>> {
        let shoeboxes = sqlx::query_as::<_, Shoebox>(
            "SELECT s.* FROM shoeboxes s
             JOIN video_shoeboxes vs ON s.id = vs.shoebox_id
             WHERE vs.video_id = $1
             ORDER BY s.name",
        )
        .bind(video_id)
        .fetch_all(&self.db)
        .await
        .map_err(AppError::Database)?;

        Ok(shoeboxes)
    }
}
