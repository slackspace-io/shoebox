use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AlbumInfo {
    pub album_name: Option<String>,
    pub album_thumbnail_asset_id: Option<String>,
    pub album_users: Option<Vec<AlbumUser>>,
    pub asset_count: Option<i64>,
    pub assets: Option<Vec<Asset>>,
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub end_date: Option<String>,
    pub has_shared_link: Option<bool>,
    pub id: String,
    pub is_activity_enabled: Option<bool>,
    pub last_modified_asset_timestamp: Option<String>,
    pub order: Option<String>,
    pub owner: Option<Owner>,
    pub shared: Option<bool>,
    pub start_date: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AlbumUser {
    pub role: String, // Possible values: ["editor", "viewer"]
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub checksum: String, // Base64 encoded SHA1 hash
    pub device_asset_id: Option<String>,
    pub device_id: Option<String>,
    pub duplicate_id: Option<String>,
    pub duration: String,
    pub exif_info: Option<ExifInfo>,
    pub file_created_at: Option<String>,
    pub file_modified_at: Option<String>,
    pub has_metadata: Option<bool>,
    pub id: String,
    pub isArchived: Option<bool>,
    pub isFavorite: Option<bool>,
    pub isOffline: Option<bool>,
    pub isTrashed: Option<bool>,
    pub libraryId: Option<String>, // Deprecated
    pub live_photo_video_id: Option<String>,
    pub local_date_time: Option<String>,
    pub originalFileName: Option<String>,
    pub originalMimeType: Option<String>,
    pub originalPath: Option<String>,
    pub owner: Option<User>,
    pub people: Option<Vec<People>>,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Deserialize, Debug)]
pub struct ExifInfo {
    pub city: Option<String>,
    pub country: Option<String>,
    pub date_time_original: Option<String>,
    pub description: Option<String>,
    pub exif_image_height: Option<f64>,
    pub exif_image_width: Option<f64>,
    pub exposure_time: Option<String>,
    pub f_number: Option<f64>,
    pub file_size_in_byte: Option<i64>,
    pub focal_length: Option<f64>,
    pub iso: Option<f64>,
    pub latitude: Option<f64>,
    pub lens_model: Option<String>,
    pub longitude: Option<f64>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub modify_date: Option<String>,
    pub orientation: Option<String>,
    pub projection_type: Option<String>,
    pub rating: Option<f64>,
    pub state: Option<String>,
    pub time_zone: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct People {
    pub birth_date: Option<String>,
    pub faces: Vec<Face>,
}

#[derive(Deserialize, Debug)]
pub struct Face {
    pub bounding_box_x1: i64,
    pub bounding_box_x2: i64,
    pub bounding_box_y1: i64,
    pub bounding_box_y2: i64,
    pub id: String,
    pub image_height: i64,
    pub image_width: i64,
    pub source_type: Option<String>, // Possible values: ["machine-learning", "exif"]
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub color: Option<String>,
    pub created_at: String,
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub updated_at: String,
    pub value: String,
    pub thumbhash: String,
    pub tag_type: String, // Possible values: ["IMAGE", "VIDEO", "AUDIO", "OTHER"]
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub avatar_color: Option<String>, // Changed to Option to handle missing values
    pub email: String,
    pub id: String,
    pub name: String,
    pub profile_changed_at: Option<String>,
    pub profile_image_path: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub avatar_color: Option<String>, // Changed to Option to handle missing values
    pub email: String,
    pub id: String,
    pub name: String,
    pub profile_changed_at: Option<String>,
    pub profile_image_path: Option<String>,
    pub owner_id: Option<String>,
}
