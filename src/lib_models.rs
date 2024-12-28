use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq )]
pub enum FileType {
    Video(String),
    Photo(String),
    Other(String),
}


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq )]
pub struct VideoMetadata {
    pub path: String,
    //include metadata struct
    pub metadata: Metadata,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq )]
pub struct Metadata {
    pub good_take: String,
    pub yearly_highlight: String,
    pub people: String,
    pub pets: String,
    //optional fields
    pub location: String,
    pub processed: String
}
