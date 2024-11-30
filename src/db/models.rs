use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::media_files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MediaFile {
    pub id: i32,
    pub name: String,
    pub path: String,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::media_files)]
pub struct NewMediaFile<'a> {
    pub name: &'a str,
    pub path: &'a str,
}
