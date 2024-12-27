use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq )]
pub enum FileType {
    Video(String),
    Photo(String),
    Other(String),
}


pub struct VideoMetadata {
    pub path: String,
    //include metadata struct
    pub metadata: Metadata,
}

pub struct Metadata {
    pub good_take: bool,
    pub yearly_highlight: bool,
    pub people: Option<Vec<String>>,
    pub pets: Option<Vec<String>>,
    //optional fields
    pub location: Option<String>,
}
