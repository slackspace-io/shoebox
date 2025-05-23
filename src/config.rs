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
    pub name: Option<String>,
    pub path: String,
    pub original_path: Option<String>,
    pub original_extension: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaSourceConfig {
    pub name: String,
    pub path: String,
    #[serde(rename = "originalPath")]
    pub original_path: Option<String>,
    #[serde(rename = "originalExtension")]
    pub original_extension: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaSourcePathsConfig {
    pub sources: Option<Vec<MediaSourceConfig>>,
    #[serde(skip)]
    pub legacy_string: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaConfig {
    #[serde(rename = "mediaSourcePaths")]
    pub media_source_paths_config: MediaSourcePathsConfig,
    pub source_paths: Vec<MediaPathConfig>,
    #[serde(rename = "exportBasePath")]
    pub export_base_path: String,
    #[serde(rename = "thumbnailPath")]
    pub thumbnail_path: String,
}

impl MediaConfig {
    // Convert MediaSourcePathsConfig to Vec<MediaPathConfig>
    pub fn convert_source_paths(&mut self) {
        if let Some(sources) = &self.media_source_paths_config.sources {
            // Convert each MediaSourceConfig to MediaPathConfig
            self.source_paths = sources.iter().map(|source| {
                MediaPathConfig {
                    name: Some(source.name.clone()),
                    path: source.path.clone(),
                    original_path: source.original_path.clone(),
                    original_extension: source.original_extension.clone(),
                }
            }).collect();
        } else if let Some(legacy_string) = &self.media_source_paths_config.legacy_string {
            // If we have a legacy string, parse it
            self.source_paths = parse_comma_separated_paths_from_string(legacy_string);
        } else {
            // If no sources are defined, use environment variables as fallback
            self.source_paths = parse_comma_separated_paths("MEDIA_SOURCE_PATHS");
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // Default configuration
        let mut config = Config {
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
                media_source_paths_config: MediaSourcePathsConfig {
                    sources: None,
                    legacy_string: env::var("MEDIA_SOURCE_PATHS").ok(),
                },
                source_paths: Vec::new(), // Will be populated in convert_source_paths
                export_base_path: env::var("EXPORT_BASE_PATH")
                    .unwrap_or_else(|_| "./exports".to_string()),
                thumbnail_path: env::var("THUMBNAIL_PATH")
                    .unwrap_or_else(|_| "./thumbnails".to_string()),
            },
        };

        // Convert media_source_paths_config to source_paths
        config.media.convert_source_paths();

        Ok(config)
    }
}

fn parse_comma_separated_paths_from_string(paths_str: &str) -> Vec<MediaPathConfig> {
    paths_str
        .split(',')
        .map(|s| s.trim().to_string())
        .map(|path_config| {
            // Check if the path contains a named section (e.g., "bmpcc:")
            if let Some(colon_pos) = path_config.find(':') {
                let name = path_config[..colon_pos].trim().to_string();
                let config_str = path_config[colon_pos + 1..].trim().to_string();

                // Split the configuration by semicolons
                let parts: Vec<&str> = config_str.split(';').collect();

                if parts.is_empty() {
                    // Invalid configuration, return a default
                    return MediaPathConfig {
                        name: Some(name),
                        path: "./media".to_string(),
                        original_path: None,
                        original_extension: None,
                    };
                }

                let path = parts[0].to_string();

                // Parse original_path if provided
                let original_path = parts.get(1)
                    .filter(|&p| !p.is_empty())
                    .map(|p| p.to_string());

                // Parse original_extension if provided
                let original_extension = parts.get(2)
                    .filter(|&e| !e.is_empty())
                    .map(|e| e.to_string());

                // If original_path is provided without extension, use the same extension as path
                let original_extension = if original_path.is_some() && original_extension.is_none() {
                    // Extract extension from path
                    std::path::Path::new(&path)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.to_string())
                } else {
                    original_extension
                };

                MediaPathConfig {
                    name: Some(name),
                    path,
                    original_path,
                    original_extension,
                }
            }
            // Backward compatibility: Check if the path contains configuration options without a name
            else if path_config.contains(';') {
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

                // If original_path is provided without extension, use the same extension as path
                let original_extension = if original_path.is_some() && original_extension.is_none() {
                    // Extract extension from path
                    std::path::Path::new(&path)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.to_string())
                } else {
                    original_extension
                };

                MediaPathConfig {
                    name: None,
                    path,
                    original_path,
                    original_extension,
                }
            } else {
                // Simple path without additional configuration
                MediaPathConfig {
                    name: None,
                    path: path_config,
                    original_path: None,
                    original_extension: None,
                }
            }
        })
        .collect()
}

fn parse_comma_separated_paths(env_var: &str) -> Vec<MediaPathConfig> {
    env::var(env_var)
        .unwrap_or_else(|_| "./media".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .map(|path_config| {
            // Check if the path contains a named section (e.g., "bmpcc:")
            if let Some(colon_pos) = path_config.find(':') {
                let name = path_config[..colon_pos].trim().to_string();
                let config_str = path_config[colon_pos + 1..].trim().to_string();

                // Split the configuration by semicolons
                let parts: Vec<&str> = config_str.split(';').collect();

                if parts.is_empty() {
                    // Invalid configuration, return a default
                    return MediaPathConfig {
                        name: Some(name),
                        path: "./media".to_string(),
                        original_path: None,
                        original_extension: None,
                    };
                }

                let path = parts[0].to_string();

                // Parse original_path if provided
                let original_path = parts.get(1)
                    .filter(|&p| !p.is_empty())
                    .map(|p| p.to_string());

                // Parse original_extension if provided
                let original_extension = parts.get(2)
                    .filter(|&e| !e.is_empty())
                    .map(|e| e.to_string());

                // If original_path is provided without extension, use the same extension as path
                let original_extension = if original_path.is_some() && original_extension.is_none() {
                    // Extract extension from path
                    std::path::Path::new(&path)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.to_string())
                } else {
                    original_extension
                };

                MediaPathConfig {
                    name: Some(name),
                    path,
                    original_path,
                    original_extension,
                }
            }
            // Backward compatibility: Check if the path contains configuration options without a name
            else if path_config.contains(';') {
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

                // If original_path is provided without extension, use the same extension as path
                let original_extension = if original_path.is_some() && original_extension.is_none() {
                    // Extract extension from path
                    std::path::Path::new(&path)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.to_string())
                } else {
                    original_extension
                };

                MediaPathConfig {
                    name: None,
                    path,
                    original_path,
                    original_extension,
                }
            } else {
                // Simple path without additional configuration
                MediaPathConfig {
                    name: None,
                    path: path_config,
                    original_path: None,
                    original_extension: None,
                }
            }
        })
        .collect()
}
