use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq )]
pub struct MediaFile {
    pub asset_type: String,
    pub path: String,
}
