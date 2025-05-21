mod scanner;
mod thumbnail;
mod video;
mod tag;
mod person;
mod export;

pub use scanner::*;
pub use thumbnail::*;
pub use video::*;
pub use tag::*;
pub use person::*;
pub use export::*;

use sqlx::{Pool, Sqlite};
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
    pub config: Config,
}
