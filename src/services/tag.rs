use sqlx::{Pool, Sqlite, Transaction, Row};
use tracing::{info, warn};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{Tag, CreateTagDto, TagUsage};

pub struct TagService {
    db: Pool<Sqlite>,
}

impl TagService {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Tag>> {
        let tags = sqlx::query_as::<_, Tag>("SELECT * FROM tags ORDER BY name")
            .fetch_all(&self.db)
            .await
            .map_err(AppError::Database)?;

        Ok(tags)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Tag> {
        let tag = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound(format!("Tag not found: {id}")),
                _ => AppError::Database(e),
            })?;

        Ok(tag)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Tag> {
        let tag = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE name = ?")
            .bind(name)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    AppError::NotFound(format!("Tag not found: {name}"))
                }
                _ => AppError::Database(e),
            })?;

        Ok(tag)
    }

    pub async fn find_or_create_by_name(
        &self,
        name: &str,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<String> {
        // Try to find existing tag
        let tag_result = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE name = ?")
            .bind(name)
            .fetch_optional(&mut **tx)
            .await
            .map_err(AppError::Database)?;

        if let Some(tag) = tag_result {
            return Ok(tag.id);
        }

        // Create new tag
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query("INSERT INTO tags (id, name, created_at) VALUES (?, ?, ?)")
            .bind(&id)
            .bind(name)
            .bind(&now)
            .execute(&mut **tx)
            .await
            .map_err(AppError::Database)?;

        info!("Created new tag: {name} ({id})");
        Ok(id)
    }

    pub async fn create(&self, dto: CreateTagDto) -> Result<Tag> {
        // Check if tag already exists
        let existing = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE name = ?")
            .bind(&dto.name)
            .fetch_optional(&self.db)
            .await
            .map_err(AppError::Database)?;

        if let Some(tag) = existing {
            return Ok(tag);
        }

        let tag = Tag::new(dto.name);

        sqlx::query("INSERT INTO tags (id, name, created_at) VALUES (?, ?, ?)")
            .bind(&tag.id)
            .bind(&tag.name)
            .bind(&tag.created_at)
            .execute(&self.db)
            .await
            .map_err(AppError::Database)?;

        info!("Created new tag: {0} ({1})", tag.name, tag.id);
        Ok(tag)
    }

    pub async fn update(&self, id: &str, new_name: &str) -> Result<Tag> {
        // Check if tag exists
        let tag = self.find_by_id(id).await?;

        // Check if the new name already exists
        let existing = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE name = ? AND id != ?")
            .bind(new_name)
            .bind(id)
            .fetch_optional(&self.db)
            .await
            .map_err(AppError::Database)?;

        if existing.is_some() {
            return Err(AppError::BadRequest(format!(
                "Tag with name '{new_name}' already exists"
            )));
        }

        // Update tag
        sqlx::query("UPDATE tags SET name = ? WHERE id = ?")
            .bind(new_name)
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(AppError::Database)?;

        info!("Updated tag: {0} -> {new_name} ({id})", tag.name);

        // Return updated tag
        let updated_tag = self.find_by_id(id).await?;
        Ok(updated_tag)
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        // Check if tag exists
        let tag = self.find_by_id(id).await?;

        // Check if tag is in use
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM video_tags WHERE tag_id = ?")
            .bind(id)
            .fetch_one(&self.db)
            .await
            .map_err(AppError::Database)?;

        if count > 0 {
            return Err(AppError::BadRequest(format!(
                "Cannot delete tag '{}' because it is used by {} videos",
                tag.name, count
            )));
        }

        // Delete tag
        sqlx::query("DELETE FROM tags WHERE id = ?")
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(AppError::Database)?;

        info!("Deleted tag: {} ({})", tag.name, id);
        Ok(())
    }

    pub async fn get_usage(&self) -> Result<Vec<TagUsage>> {
        let rows = sqlx::query(
            "SELECT t.id, t.name, COUNT(vt.video_id) as video_count
             FROM tags t
             LEFT JOIN video_tags vt ON t.id = vt.tag_id
             GROUP BY t.id
             ORDER BY t.name",
        )
        .fetch_all(&self.db)
        .await
        .map_err(AppError::Database)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(TagUsage {
                id: row.get("id"),
                name: row.get("name"),
                video_count: row.get("video_count"),
            });
        }

        Ok(results)
    }

    pub async fn cleanup_unused(&self) -> Result<usize> {
        let result = sqlx::query(
            "DELETE FROM tags
             WHERE id NOT IN (SELECT DISTINCT tag_id FROM video_tags)",
        )
        .execute(&self.db)
        .await
        .map_err(AppError::Database)?;

        let count = result.rows_affected() as usize;
        if count > 0 {
            info!("Cleaned up {} unused tags", count);
        }

        Ok(count)
    }
}
