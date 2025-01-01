use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq )]
pub struct MediaFile {
    pub asset_type: String,
    pub path: String,
    pub file_name: String,
    pub creation_date: String,
    pub discovery_date: String,
}

