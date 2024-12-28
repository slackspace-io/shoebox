use leptos::logging::log;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Connection, Error, Result, ToSql};
use crate::lib_models::{FileType, Metadata, VideoMetadata};
use crate::models::MediaFile;

pub fn create_table_if_not_exist() -> Result<()> {
    let conn = Connection::open("data.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS media_assets (
            id INTEGER PRIMARY KEY,
            asset_type TEXT NOT NULL,
            path TEXT NOT NULL,
            good_take BOOLEAN,
            yearly_highlight BOOLEAN,
            people TEXT,
            pets TEXT,
            location TEXT,
            processed BOOLEAN DEFAULT FALSE
        )",
        [],
    )?;
    Ok(())
}



pub fn check_if_media_asset_exists(media_file: MediaFile) -> Result<bool> {
    let conn = Connection::open("data.db")?;
    let mut stmt = conn.prepare("SELECT * FROM media_assets WHERE path = ?1")?;
    let media_assets = stmt.query_map(&[&media_file.path], |row| {
        Ok(MediaFile {
            asset_type: row.get(1)?,
            path: row.get(2)?,
        })
    })?;
    let mut media_assets_vec = Vec::new();
    for media_asset in media_assets {
        media_assets_vec.push(media_asset?);
    }
    if media_assets_vec.len() > 0 {
        log!("Media asset exists");
        Ok(true)
    } else {
        log!("Media asset does not exist");
        Ok(false)
    }
}


pub fn update_video_metadata(metadata: VideoMetadata) -> Result<usize> {
    let conn = Connection::open("data.db")?;
    log!("Updating video metadata");
    conn.execute(
        "UPDATE media_assets SET good_take = ?1, yearly_highlight = ?2, people = ?3, pets = ?4, location = ?5, processed = ?6 WHERE path = ?7",
        &[&metadata.metadata.good_take, &metadata.metadata.yearly_highlight, &metadata.metadata.people, &metadata.metadata.pets, &metadata.metadata.location, &metadata.metadata.processed, &metadata.path],
    )
}


pub fn insert_media_asset(media_file: MediaFile) -> Result<usize> {
    let conn = Connection::open("data.db")?;
    if check_if_media_asset_exists(media_file.clone())? {
        return Ok(0);
    } else {
    conn.execute(
        "INSERT INTO media_assets (asset_type, path) VALUES (?1, ?2)",
        &[&media_file.asset_type, &media_file.path],
    )
    }
}

pub fn return_all_media_assets() -> Result<Vec<MediaFile>> {
    let conn = Connection::open("data.db")?;
    let mut stmt = conn.prepare("SELECT * FROM media_assets WHERE processed = FALSE")?;
    let media_assets = stmt.query_map([], |row| {
        Ok(MediaFile {
            asset_type: row.get(1)?,
            path: row.get(2)?,
        })
    })?;
    let mut media_assets_vec = Vec::new();
    for media_asset in media_assets {
        media_assets_vec.push(media_asset?);
    }
    Ok(media_assets_vec)
}



