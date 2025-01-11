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
    pub deviceAssetId: Option<String>,
    pub deviceId: Option<String>,
    pub duplicateId: Option<String>,
    pub duration: String,
    pub exifInfo: Option<ExifInfo>,
    pub fileCreatedAt: Option<String>,
    pub fileModifiedAt: Option<String>,
    pub hasMetadata: Option<bool>,
    pub id: String,
    pub isArchived: Option<bool>,
    pub isFavorite: Option<bool>,
    pub isOffline: Option<bool>,
    pub isTrashed: Option<bool>,
    pub libraryId: Option<String>, // Deprecated
    pub livePhotoVideoId: Option<String>,
    pub localDateTime: Option<String>,
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
    pub dateTimeOriginal: Option<String>,
    pub description: Option<String>,
    pub exifImageHeight: Option<f64>,
    pub exifImageWidth: Option<f64>,
    pub exposureTime: Option<String>,
    pub fNumber: Option<f64>,
    pub fileSizeInByte: Option<i64>,
    pub focalLength: Option<f64>,
    pub iso: Option<f64>,
    pub latitude: Option<f64>,
    pub lensModel: Option<String>,
    pub longitude: Option<f64>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub modifyDate: Option<String>,
    pub orientation: Option<String>,
    pub projectionType: Option<String>,
    pub rating: Option<f64>,
    pub state: Option<String>,
    pub timeZone: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct People {
    pub id: String,
    pub name: String,
    pub birthDate: Option<String>,
    pub faces: Vec<Face>,
}

#[derive(Deserialize, Debug)]
pub struct Face {
    pub boundingBoxX1: i64,
    pub boundingBoxX2: i64,
    pub boundingBoxY1: i64,
    pub boundingBoxY2: i64,
    pub id: String,
    pub imageHeight: i64,
    pub imageWidth: i64,
    pub sourceType: Option<String>, // Possible values: ["machine-learning", "exif"]
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub color: Option<String>,
    pub createdAt: String,
    pub id: String,
    pub name: String,
    pub parentId: Option<String>,
    pub updatedAt: String,
    pub value: String,
    pub thumbhash: String,
    pub tagType: String, // Possible values: ["IMAGE", "VIDEO", "AUDIO", "OTHER"]
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
