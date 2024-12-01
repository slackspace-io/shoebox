use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Connection, Error, Result, ToSql};
use crate::lib_models::FileType;
use crate::models::MediaFile;

pub fn create_table_if_not_exist() -> Result<()> {
    let conn = Connection::open("data.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS media_assets (
            id INTEGER PRIMARY KEY,
            asset_type TEXT NOT NULL,
            path TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}


pub fn insert_media_asset(media_file: MediaFile) -> Result<usize> {
    let conn = Connection::open("data.db")?;
    conn.execute(
        "INSERT INTO media_assets (asset_type, path) VALUES (?1, ?2)",
        &[&media_file.asset_type, &media_file.path],
    )
}

pub fn return_all_media_assets() -> Result<Vec<MediaFile>> {
    let conn = Connection::open("data.db")?;
    let mut stmt = conn.prepare("SELECT * FROM media_assets")?;
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



