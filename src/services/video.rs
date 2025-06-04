use sqlx::{Pool, Postgres, Row};
use tracing::{info, error};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{Video, CreateVideoDto, UpdateVideoDto, VideoWithMetadata, VideoSearchParams};
use crate::services::tag::TagService;
use crate::services::person::PersonService;
use crate::services::thumbnail::ThumbnailService;

pub struct VideoService {
    db: Pool<Postgres>,
    tag_service: TagService,
    person_service: PersonService,
    thumbnail_service: ThumbnailService,
}

impl VideoService {
    pub fn new(
        db: Pool<Postgres>,
        tag_service: TagService,
        person_service: PersonService,
        thumbnail_service: ThumbnailService,
    ) -> Self {
        Self {
            db,
            tag_service,
            person_service,
            thumbnail_service,
        }
    }

    // Helper method to transform thumbnail paths to web-compatible paths
    fn transform_thumbnail_path(&self, thumbnail_path: Option<String>) -> Option<String> {
        thumbnail_path.map(|path| {
            // Extract just the filename from the path
            let path = std::path::Path::new(&path);
            let filename = path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");

            // Return a web-compatible path
            format!("/app/thumbnails/{filename}")
        })
    }

    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<Video>> {
        let mut videos = sqlx::query_as::<_, Video>(
            "SELECT * FROM videos ORDER BY created_date DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await
        .map_err(AppError::Database)?;

        // Transform thumbnail paths
        for video in &mut videos {
            video.thumbnail_path = self.transform_thumbnail_path(video.thumbnail_path.clone());
        }

        Ok(videos)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Video> {
        let mut video = sqlx::query_as::<_, Video>("SELECT * FROM videos WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound(format!("Video not found: {id}")),
                _ => AppError::Database(e),
            })?;

        // Transform thumbnail path
        video.thumbnail_path = self.transform_thumbnail_path(video.thumbnail_path.clone());

        Ok(video)
    }

    pub async fn find_by_path(&self, path: &str) -> Result<Video> {
        let mut video = sqlx::query_as::<_, Video>("SELECT * FROM videos WHERE file_path = $1")
            .bind(path)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound(format!("Video not found: {path}")),
                _ => AppError::Database(e),
            })?;

        // Transform thumbnail path
        video.thumbnail_path = self.transform_thumbnail_path(video.thumbnail_path.clone());

        Ok(video)
    }

    pub async fn find_with_metadata(&self, id: &str) -> Result<VideoWithMetadata> {
        let video = self.find_by_id(id).await?;

        // Get tags for this video
        let tags = sqlx::query_scalar::<_, String>(
            "SELECT t.name FROM tags t
             JOIN video_tags vt ON t.id = vt.tag_id
             WHERE vt.video_id = $1"
        )
        .bind(id)
        .fetch_all(&self.db)
        .await
        .map_err(AppError::Database)?;

        // Get people for this video
        let people = sqlx::query_scalar::<_, String>(
            "SELECT p.name FROM people p
             JOIN video_people vp ON p.id = vp.person_id
             WHERE vp.video_id = $1"
        )
        .bind(id)
        .fetch_all(&self.db)
        .await
        .map_err(AppError::Database)?;

        // Note: find_by_id already transforms the thumbnail path

        Ok(VideoWithMetadata {
            video,
            tags,
            people,
        })
    }

    pub async fn create(&self, dto: CreateVideoDto) -> Result<Video> {
        let mut tx = self.db.begin().await.map_err(AppError::Database)?;

        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().naive_utc();

        // Insert video
        sqlx::query(
            "INSERT INTO videos (id, file_path, file_name, title, description, created_date, file_size, thumbnail_path, rating, duration, original_file_path, location, event, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14::timestamp, $15::timestamp)"
        )
        .bind(&id)
        .bind(&dto.file_path)
        .bind(&dto.file_name)
        .bind(&dto.title)
        .bind(&dto.description)
        .bind(&dto.created_date)
        .bind(&dto.file_size)
        .bind(&dto.thumbnail_path)
        .bind(&dto.rating)
        .bind(&dto.duration)
        .bind(&dto.original_file_path)
        .bind(&dto.location)
        .bind(&dto.event)
        .bind(&now)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        // Add tags
        for tag_name in &dto.tags {
            let tag_id = self.tag_service.find_or_create_by_name(tag_name, &mut tx).await?;

            sqlx::query("INSERT INTO video_tags (video_id, tag_id, created_at) VALUES ($1, $2, $3::timestamp)")
                .bind(&id)
                .bind(&tag_id)
                .bind(&now)
                .execute(&mut *tx)
                .await
                .map_err(AppError::Database)?;
        }

        // Add people
        for person_name in &dto.people {
            let person_id = self.person_service.find_or_create_by_name(person_name, &mut tx).await?;

            sqlx::query("INSERT INTO video_people (video_id, person_id, created_at) VALUES ($1, $2, $3::timestamp)")
                .bind(&id)
                .bind(&person_id)
                .bind(&now)
                .execute(&mut *tx)
                .await
                .map_err(AppError::Database)?;
        }

        tx.commit().await.map_err(AppError::Database)?;

        // Return the created video (find_by_id already transforms the thumbnail path)
        self.find_by_id(&id).await
    }

    pub async fn update(&self, id: &str, dto: UpdateVideoDto) -> Result<Video> {
        let mut tx = self.db.begin().await.map_err(AppError::Database)?;

        // Check if video exists
        let _video = self.find_by_id(id).await?;
        let now = chrono::Utc::now().naive_utc();

        // Update video fields
        let mut query = "UPDATE videos SET updated_at = $1::timestamp".to_string();
        let mut params: Vec<String> = vec![now.to_string()];
        let mut param_count = 1;

        if let Some(title) = &dto.title {
            param_count += 1;
            query.push_str(&format!(", title = ${}", param_count));
            params.push(title.clone());
        }

        if let Some(description) = &dto.description {
            param_count += 1;
            query.push_str(&format!(", description = ${}", param_count));
            params.push(description.clone());
        }

        if let Some(rating) = dto.rating {
            param_count += 1;
            query.push_str(&format!(", rating = ${}::integer", param_count));
            params.push(rating.to_string());
        }

        if let Some(location) = &dto.location {
            param_count += 1;
            query.push_str(&format!(", location = ${}", param_count));
            params.push(location.clone());
        }

        if let Some(event) = &dto.event {
            param_count += 1;
            query.push_str(&format!(", event = ${}", param_count));
            params.push(event.clone());
        }

        param_count += 1;
        query.push_str(&format!(" WHERE id = ${}", param_count));
        params.push(id.to_string());

        let mut query_builder = sqlx::query(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }

        query_builder
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        // Update tags if provided
        if let Some(tags) = &dto.tags {
            // Remove existing tags
            sqlx::query("DELETE FROM video_tags WHERE video_id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(AppError::Database)?;

            // Add new tags
            for tag_name in tags {
                let tag_id = self.tag_service.find_or_create_by_name(tag_name, &mut tx).await?;

                sqlx::query("INSERT INTO video_tags (video_id, tag_id, created_at) VALUES ($1, $2, $3::timestamp)")
                    .bind(id)
                    .bind(&tag_id)
                    .bind(&now)
                    .execute(&mut *tx)
                    .await
                    .map_err(AppError::Database)?;
            }
        }

        // Update people if provided
        if let Some(people) = &dto.people {
            // Remove existing people
            sqlx::query("DELETE FROM video_people WHERE video_id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(AppError::Database)?;

            // Add new people
            for person_name in people {
                let person_id = self.person_service.find_or_create_by_name(person_name, &mut tx).await?;

                sqlx::query("INSERT INTO video_people (video_id, person_id, created_at) VALUES ($1, $2, $3::timestamp)")
                    .bind(id)
                    .bind(&person_id)
                    .bind(&now)
                    .execute(&mut *tx)
                    .await
                    .map_err(AppError::Database)?;
            }
        }

        tx.commit().await.map_err(AppError::Database)?;

        // Return the updated video (find_by_id already transforms the thumbnail path)
        self.find_by_id(id).await
    }

    pub async fn update_technical_metadata(&self, id: &str, file_size: Option<i64>, duration: Option<i64>, created_date: Option<String>, thumbnail_path: Option<String>, original_file_path: Option<String>, exif_data: Option<serde_json::Value>) -> Result<Video> {
        let mut tx = self.db.begin().await.map_err(AppError::Database)?;

        // Check if video exists
        let _video = self.find_by_id(id).await?;
        let now = chrono::Utc::now().naive_utc();

        // Update video fields
        let mut query = "UPDATE videos SET updated_at = $1::timestamp".to_string();
        let mut params: Vec<String> = vec![now.to_string()];
        let mut param_count = 1;

        if let Some(size) = file_size {
            param_count += 1;
            query.push_str(&format!(", file_size = ${}::bigint", param_count));
            params.push(size.to_string());
        }

        if let Some(dur) = duration {
            param_count += 1;
            query.push_str(&format!(", duration = ${}::bigint", param_count));
            params.push(dur.to_string());
        }

        if let Some(date) = &created_date {
            param_count += 1;
            query.push_str(&format!(", created_date = ${}", param_count));
            params.push(date.clone());
        }

        if let Some(thumb) = &thumbnail_path {
            param_count += 1;
            query.push_str(&format!(", thumbnail_path = ${}", param_count));
            params.push(thumb.clone());
        }

        if let Some(orig) = &original_file_path {
            param_count += 1;
            query.push_str(&format!(", original_file_path = ${}", param_count));
            params.push(orig.clone());
        }

        if let Some(exif) = &exif_data {
            param_count += 1;
            query.push_str(&format!(", exif_data = ${}::jsonb", param_count));
            params.push(exif.to_string());
        }

        param_count += 1;
        query.push_str(&format!(" WHERE id = ${}", param_count));
        params.push(id.to_string());

        info!("Executing query: {} with params: {:?}", query, params);

        let mut query_builder = sqlx::query(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }

        query_builder
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        tx.commit().await.map_err(AppError::Database)?;

        // Return the updated video (find_by_id already transforms the thumbnail path)
        self.find_by_id(id).await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let video = self.find_by_id(id).await?;

        // Delete the video record
        sqlx::query("DELETE FROM videos WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(AppError::Database)?;

        // Delete the thumbnail if it exists
        if let Some(thumbnail_path) = video.thumbnail_path {
            if let Err(e) = self.thumbnail_service.delete_thumbnail(&thumbnail_path).await {
                error!("Failed to delete thumbnail: {}", e);
                // Continue with deletion even if thumbnail deletion fails
            }
        }

        info!("Deleted video: {}", id);
        Ok(())
    }

    pub async fn bulk_update(&self, video_ids: Vec<String>, update_dto: UpdateVideoDto) -> Result<Vec<Video>> {
        let mut updated_videos = Vec::new();

        for id in video_ids {
            match self.update(&id, update_dto.clone()).await {
                Ok(video) => updated_videos.push(video),
                Err(e) => {
                    error!("Failed to update video {}: {}", id, e);
                    // Continue with other videos even if one fails
                }
            }
        }

        Ok(updated_videos)
    }

    pub async fn search(&self, params: VideoSearchParams) -> Result<Vec<VideoWithMetadata>> {
        let mut conditions = Vec::<String>::new();
        let mut query_params = Vec::new();
        let mut param_count = 0;

        // Base query
        let mut query = "
            SELECT v.*,
                   string_agg(DISTINCT t.name, ',') as tags,
                   string_agg(DISTINCT p.name, ',') as people
            FROM videos v
            LEFT JOIN video_tags vt ON v.id = vt.video_id
            LEFT JOIN tags t ON vt.tag_id = t.id
            LEFT JOIN video_people vp ON v.id = vp.video_id
            LEFT JOIN people p ON vp.person_id = p.id
        ".to_string();

        // Add search conditions
        if let Some(search_query) = &params.query {
            param_count += 1;
            let title_param = format!("${}", param_count);
            param_count += 1;
            let desc_param = format!("${}", param_count);
            param_count += 1;
            let tag_param = format!("${}", param_count);
            param_count += 1;
            let person_param = format!("${}", param_count);

            conditions.push(format!("(v.title LIKE {title_param} OR v.description LIKE {desc_param} OR t.name LIKE {tag_param} OR p.name LIKE {person_param})"));
            let like_param = format!("%{search_query}%");
            query_params.push(like_param.clone());
            query_params.push(like_param.clone());
            query_params.push(like_param.clone());
            query_params.push(like_param);
        }

        if let Some(tags) = &params.tags {
            if !tags.is_empty() {
                // For each tag, we need a separate subquery to ensure ALL tags are present
                for tag in tags {
                    param_count += 1;
                    let condition = format!("v.id IN (
                        SELECT video_id FROM video_tags
                        JOIN tags ON video_tags.tag_id = tags.id
                        WHERE tags.name = ${}
                    )", param_count);
                    conditions.push(condition);
                    query_params.push(tag.clone());
                }
            }
        }

        if let Some(people) = &params.people {
            if !people.is_empty() {
                // For each person, we need a separate subquery to ensure ALL people are present
                for person in people {
                    param_count += 1;
                    let condition = format!("v.id IN (
                        SELECT video_id FROM video_people
                        JOIN people ON video_people.person_id = people.id
                        WHERE people.name = ${}
                    )", param_count);
                    conditions.push(condition);
                    query_params.push(person.clone());
                }
            }
        }

        if let Some(rating) = params.rating {
            param_count += 1;
            conditions.push(format!("v.rating >= ${}::integer", param_count));
            query_params.push(rating.to_string());
        }

        if let Some(location) = &params.location {
            param_count += 1;
            conditions.push(format!("v.location LIKE ${}", param_count));
            query_params.push(format!("%{}%", location));
        }

        if let Some(event) = &params.event {
            param_count += 1;
            conditions.push(format!("v.event LIKE ${}", param_count));
            query_params.push(format!("%{}%", event));
        }

        if let Some(true) = params.unreviewed {
            conditions.push("(v.rating IS NULL AND v.description IS NULL AND v.location IS NULL AND v.event IS NULL AND NOT EXISTS (SELECT 1 FROM video_tags WHERE video_id = v.id) AND NOT EXISTS (SELECT 1 FROM video_people WHERE video_id = v.id))".to_string());
        }

        if let Some(start_date) = &params.start_date {
            param_count += 1;
            conditions.push(format!("date(v.created_date) >= date(${}) ", param_count));
            query_params.push(start_date.clone());
        }

        if let Some(end_date) = &params.end_date {
            param_count += 1;
            conditions.push(format!("date(v.created_date) <= date(${}) ", param_count));
            query_params.push(end_date.clone());
        }

        if let Some(min_duration) = params.min_duration {
            // Convert from seconds to milliseconds
            let min_duration_ms = min_duration * 1000;
            param_count += 1;
            conditions.push(format!("v.duration >= ${}::bigint", param_count));
            query_params.push(min_duration_ms.to_string());
        }

        if let Some(max_duration) = params.max_duration {
            // Convert from seconds to milliseconds
            let max_duration_ms = max_duration * 1000;
            param_count += 1;
            conditions.push(format!("v.duration <= ${}::bigint", param_count));
            query_params.push(max_duration_ms.to_string());
        }

        // Add WHERE clause if conditions exist
        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        // Add GROUP BY clause
        query.push_str(" GROUP BY v.id");

        // Add ORDER BY clause
        if let Some(sort_by) = &params.sort_by {
            let order = params.sort_order.as_deref().unwrap_or("ASC");
            let order = if order.to_uppercase() == "DESC" { "DESC" } else { "ASC" };

            match sort_by.as_str() {
                "duration" => query.push_str(&format!(" ORDER BY v.duration {order}")),
                "title" => query.push_str(&format!(" ORDER BY v.title {order}")),
                "rating" => query.push_str(&format!(" ORDER BY v.rating {order}")),
                "file_size" => query.push_str(&format!(" ORDER BY v.file_size {order}")),
                "created_date" => query.push_str(&format!(" ORDER BY to_timestamp(v.created_date, 'YYYY-MM-DD\"T\"HH24:MI:SS.MS\"Z\"') {order}")),
                _ => query.push_str(" ORDER BY v.created_date DESC"),
            }
        } else {
            query.push_str(" ORDER BY to_timestamp(v.created_date, 'YYYY-MM-DD\"T\"HH24:MI:SS.MS\"Z\"') DESC");
        }

        if let Some(limit) = params.limit {
            param_count += 1;
            query.push_str(&format!(" LIMIT ${}::integer", param_count));
            query_params.push(limit.to_string());
        } else {
            query.push_str(" LIMIT 100"); // Default limit
        }

        if let Some(offset) = params.offset {
            param_count += 1;
            query.push_str(&format!(" OFFSET ${}::bigint", param_count));
            query_params.push(offset.to_string());
        }

        // Execute query
        let mut query_builder = sqlx::query(&query);
        for param in query_params {
            query_builder = query_builder.bind(param);
        }

        let rows = query_builder
            .fetch_all(&self.db)
            .await
            .map_err(AppError::Database)?;

        // Convert rows to VideoWithMetadata
        let mut results = Vec::new();
        for row in rows {
            let id: String = row.get("id");
            let mut video = Video {
                id: id.clone(),
                file_path: row.get("file_path"),
                file_name: row.get("file_name"),
                title: row.get("title"),
                description: row.get("description"),
                created_date: row.get("created_date"),
                file_size: row.get("file_size"),
                thumbnail_path: row.get("thumbnail_path"),
                rating: row.get("rating"),
                duration: row.get("duration"),
                original_file_path: row.get("original_file_path"),
                exif_data: row.get("exif_data"),
                location: row.get("location"),
                event: row.get("event"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };

            // Transform thumbnail path
            video.thumbnail_path = self.transform_thumbnail_path(video.thumbnail_path.clone());

            let tags_str: Option<String> = row.get("tags");
            let tags = tags_str
                .map(|s| s.split(',').map(|t| t.to_string()).collect())
                .unwrap_or_default();

            let people_str: Option<String> = row.get("people");
            let people = people_str
                .map(|s| s.split(',').map(|p| p.to_string()).collect())
                .unwrap_or_default();

            results.push(VideoWithMetadata {
                video,
                tags,
                people,
            });
        }

        Ok(results)
    }
}
