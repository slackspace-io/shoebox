use diesel::prelude::*;
use dotenvy::dotenv;
use diesel::associations::HasTable;
use crate::models::NewMediaFile;
use crate::models::MediaFile;
use crate::schema::media_files::dsl::media_files;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = ".database.db";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_media_file(asset_type: &str, path: &str) -> usize {
    let conn = &mut establish_connection();
    let new_media_file = NewMediaFile { asset_type, path };


    diesel::insert_into(media_files::table())
        .values(&new_media_file)
        .execute(conn)
        .expect("Error saving new media file")


}
pub fn get_all_media_files() -> Vec<MediaFile> {
    let conn = &mut establish_connection();
    media_files.load::<MediaFile>(conn).expect("Error loading media files")
}
