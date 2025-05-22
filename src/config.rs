use serde::{Deserialize, Serialize};
use std::env;
use anyhow::Result;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub media: MediaConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaPathConfig {
    pub path: String,
    pub original_path: Option<String>,
    pub original_extension: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaConfig {
    pub source_paths: Vec<MediaPathConfig>,
    pub export_base_path: String,
    pub thumbnail_path: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        // Default configuration
        let config = Config {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .unwrap_or(3000),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data.db".to_string()),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .unwrap_or(5),
            },
            media: MediaConfig {
                source_paths: parse_comma_separated_paths("MEDIA_SOURCE_PATHS"),
                export_base_path: env::var("EXPORT_BASE_PATH")
                    .unwrap_or_else(|_| "./exports".to_string()),
                thumbnail_path: env::var("THUMBNAIL_PATH")
                    .unwrap_or_else(|_| "./thumbnails".to_string()),
            },
        };

        Ok(config)
    }
}

fn parse_comma_separated_paths(env_var: &str) -> Vec<MediaPathConfig> {
    env::var(env_var)
        .unwrap_or_else(|_| "./media".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .map(|path_config| {
            // Check if the path contains configuration options
            if path_config.contains(';') {
                let parts: Vec<&str> = path_config.split(';').collect();
                let path = parts[0].to_string();

                // Parse original_path if provided
                let original_path = parts.get(1)
                    .filter(|&p| !p.is_empty())
                    .map(|p| p.to_string());

                // Parse original_extension if provided
                let original_extension = parts.get(2)
                    .filter(|&e| !e.is_empty())
                    .map(|e| e.to_string());

                MediaPathConfig {
                    path,
                    original_path,
                    original_extension,
                }
            } else {
                // Simple path without additional configuration
                MediaPathConfig {
                    path: path_config,
                    original_path: None,
                    original_extension: None,
                }
            }
        })
        .collect()
}
