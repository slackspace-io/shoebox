use sqlx::{Pool, Sqlite, Transaction, Row};
use tracing::{info, warn};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{Person, CreatePersonDto, PersonUsage};

pub struct PersonService {
    db: Pool<Sqlite>,
}

impl PersonService {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Person>> {
        let people = sqlx::query_as::<_, Person>("SELECT * FROM people ORDER BY name")
            .fetch_all(&self.db)
            .await
            .map_err(AppError::Database)?;

        Ok(people)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Person> {
        let person = sqlx::query_as::<_, Person>("SELECT * FROM people WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound(format!("Person not found: {id}")),
                _ => AppError::Database(e),
            })?;

        Ok(person)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Person> {
        let person = sqlx::query_as::<_, Person>("SELECT * FROM people WHERE name = ?")
            .bind(name)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    AppError::NotFound(format!("Person not found: {name}"))
                }
                _ => AppError::Database(e),
            })?;

        Ok(person)
    }

    pub async fn find_or_create_by_name(
        &self,
        name: &str,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<String> {
        // Try to find existing person
        let person_result = sqlx::query_as::<_, Person>("SELECT * FROM people WHERE name = ?")
            .bind(name)
            .fetch_optional(&mut **tx)
            .await
            .map_err(AppError::Database)?;

        if let Some(person) = person_result {
            return Ok(person.id);
        }

        // Create new person
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query("INSERT INTO people (id, name, created_at) VALUES (?, ?, ?)")
            .bind(&id)
            .bind(name)
            .bind(&now)
            .execute(&mut **tx)
            .await
            .map_err(AppError::Database)?;

        info!("Created new person: {name} ({id})");
        Ok(id)
    }

    pub async fn create(&self, dto: CreatePersonDto) -> Result<Person> {
        // Check if person already exists
        let existing = sqlx::query_as::<_, Person>("SELECT * FROM people WHERE name = ?")
            .bind(&dto.name)
            .fetch_optional(&self.db)
            .await
            .map_err(AppError::Database)?;

        if let Some(person) = existing {
            return Ok(person);
        }

        let person = Person::new(dto.name);

        sqlx::query("INSERT INTO people (id, name, created_at) VALUES (?, ?, ?)")
            .bind(&person.id)
            .bind(&person.name)
            .bind(&person.created_at)
            .execute(&self.db)
            .await
            .map_err(AppError::Database)?;

        info!("Created new person: {} ({})", person.name, person.id);
        Ok(person)
    }

    pub async fn update(&self, id: &str, new_name: &str) -> Result<Person> {
        // Check if person exists
        let person = self.find_by_id(id).await?;

        // Check if the new name already exists
        let existing = sqlx::query_as::<_, Person>("SELECT * FROM people WHERE name = ? AND id != ?")
            .bind(new_name)
            .bind(id)
            .fetch_optional(&self.db)
            .await
            .map_err(AppError::Database)?;

        if existing.is_some() {
            return Err(AppError::BadRequest(format!(
                "Person with name '{new_name}' already exists"
            )));
        }

        // Update person
        sqlx::query("UPDATE people SET name = ? WHERE id = ?")
            .bind(new_name)
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(AppError::Database)?;

        info!("Updated person: {0} -> {new_name} ({id})", person.name);

        // Return updated person
        let updated_person = self.find_by_id(id).await?;
        Ok(updated_person)
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        // Check if person exists
        let person = self.find_by_id(id).await?;

        // Check if person is in use
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM video_people WHERE person_id = ?")
            .bind(id)
            .fetch_one(&self.db)
            .await
            .map_err(AppError::Database)?;

        if count > 0 {
            return Err(AppError::BadRequest(format!(
                "Cannot delete person '{}' because they appear in {} videos",
                person.name, count
            )));
        }

        // Delete person
        sqlx::query("DELETE FROM people WHERE id = ?")
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(AppError::Database)?;

        info!("Deleted person: {} ({})", person.name, id);
        Ok(())
    }

    pub async fn get_usage(&self) -> Result<Vec<PersonUsage>> {
        let rows = sqlx::query(
            "SELECT p.id, p.name, COUNT(vp.video_id) as video_count
             FROM people p
             LEFT JOIN video_people vp ON p.id = vp.person_id
             GROUP BY p.id
             ORDER BY p.name",
        )
        .fetch_all(&self.db)
        .await
        .map_err(AppError::Database)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(PersonUsage {
                id: row.get("id"),
                name: row.get("name"),
                video_count: row.get("video_count"),
            });
        }

        Ok(results)
    }

    pub async fn cleanup_unused(&self) -> Result<usize> {
        let result = sqlx::query(
            "DELETE FROM people
             WHERE id NOT IN (SELECT DISTINCT person_id FROM video_people)",
        )
        .execute(&self.db)
        .await
        .map_err(AppError::Database)?;

        let count = result.rows_affected() as usize;
        if count > 0 {
            info!("Cleaned up {} unused people", count);
        }

        Ok(count)
    }
}
