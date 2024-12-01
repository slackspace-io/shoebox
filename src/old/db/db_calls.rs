use diesel::{Connection, RunQueryDsl, SqliteConnection};
use diesel::associations::HasTable;
use crate::db::models::NewMediaFile;
use crate::db::models::MediaFile;
use crate::schema::media_files::dsl::media_files;
use dotenvy::dotenv;

pub fn estssablish_connection() -> SqliteConnection {
    dotenv().ok();

    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = ".database.db";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


