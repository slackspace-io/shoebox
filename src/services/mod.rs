mod scanner;
mod thumbnail;
mod video;
mod tag;
mod person;
mod export;
mod location;
mod event;

pub use scanner::*;
pub use thumbnail::*;
pub use video::*;
pub use tag::*;
pub use person::*;
pub use export::*;
pub use location::*;
pub use event::*;

use sqlx::{Pool, Sqlite};
use crate::config::Config;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Represents the current status of a scan operation
#[derive(Clone, Debug)]
pub struct ScanStatus {
    pub in_progress: bool,
    pub new_videos_count: usize,
    pub updated_videos_count: usize,
}

impl Default for ScanStatus {
    fn default() -> Self {
        Self {
            in_progress: false,
            new_videos_count: 0,
            updated_videos_count: 0,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
    pub config: Config,
    pub scan_status: Arc<RwLock<ScanStatus>>,
}
