use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use std::fs;
use std::path::Path;
use tracing::info;

use crate::config::Config;
use crate::error::{AppError, Result};

pub async fn init_db(config: &Config) -> Result<Pool<Sqlite>> {
    let db_url = &config.database.url;

    // Create database if it doesn't exist
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        info!("Creating database at {}", db_url);
        Sqlite::create_database(db_url).await.map_err(|e| {
            AppError::Database(sqlx::Error::Configuration(
                format!("Failed to create database: {}", e).into(),
            ))
        })?;
    }

    // Create migrations directory if it doesn't exist
    let migrations_dir = Path::new("./migrations");
    if !migrations_dir.exists() {
        info!("Creating migrations dir");
        fs::create_dir_all(migrations_dir).map_err(|e| {
            AppError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create migrations directory: {}", e),
            ))
        })?;

        // Create initial migration file
        info!("create initial migration");
        create_initial_migration(migrations_dir)?;
    }

    // Connect to the database
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePool::connect_with(options.filename(db_url))
        .await
        .map_err(AppError::Database)?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| AppError::Database(sqlx::Error::Migrate(Box::new(e))))?;

    info!("Database initialized successfully");
    Ok(pool)
}

fn create_initial_migration(migrations_dir: &Path) -> Result<()> {
    let migration_file = migrations_dir.join("20240101000000_initial_schema.sql");
    let migration_content = r#"-- Initial schema for family video organizer
-- Up migration

-- Videos table
CREATE TABLE IF NOT EXISTS videos (
    id TEXT PRIMARY KEY NOT NULL,
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    title TEXT,
    description TEXT,
    created_date TEXT,
    file_size INTEGER,
    thumbnail_path TEXT,
    rating INTEGER CHECK (rating BETWEEN 1 AND 5 OR rating IS NULL),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Tags table
CREATE TABLE IF NOT EXISTS tags (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- People table
CREATE TABLE IF NOT EXISTS people (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Video-Tag relationship table
CREATE TABLE IF NOT EXISTS video_tags (
    video_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (video_id, tag_id),
    FOREIGN KEY (video_id) REFERENCES videos (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);

-- Video-People relationship table
CREATE TABLE IF NOT EXISTS video_people (
    video_id TEXT NOT NULL,
    person_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (video_id, person_id),
    FOREIGN KEY (video_id) REFERENCES videos (id) ON DELETE CASCADE,
    FOREIGN KEY (person_id) REFERENCES people (id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_videos_file_path ON videos (file_path);
CREATE INDEX IF NOT EXISTS idx_videos_created_date ON videos (created_date);
CREATE INDEX IF NOT EXISTS idx_tags_name ON tags (name);
CREATE INDEX IF NOT EXISTS idx_people_name ON people (name);

-- Down migration
-- DROP TABLE IF EXISTS video_people;
-- DROP TABLE IF EXISTS video_tags;
-- DROP TABLE IF EXISTS people;
-- DROP TABLE IF EXISTS tags;
-- DROP TABLE IF EXISTS videos;
"#;

    fs::write(migration_file, migration_content).map_err(AppError::Io)?;
    info!("Created initial migration file");
    Ok(())
}
