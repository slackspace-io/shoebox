use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq )]
pub enum FileType {
    Video(String),
    Photo(String),
    Other(String),
}

