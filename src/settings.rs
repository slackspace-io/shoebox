use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use serde_derive::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseConfig,
    pub paths: Vec<PathConfig>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub database_ip_port: String,
    pub database_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PathConfig {
    pub root_path: String,
    pub name: String,
    pub description: String,
}

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        //        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            //.add_source(
            //    File::with_name(&format!("examples/hierarchical-env/config/{run_mode}"))
            //        .required(false),
            //)
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("examples/hierarchical-env/config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("SHOEBOX"))
            // You may also programmatically change settings
            // .set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        //        println!("debug: {:?}", s.get_bool("debug"));
        println!("server: {:?}", s.get::<String>("paths"));
        println!(
            "database: {:?}",
            s.get::<String>("database.database_ip_port")
        );
        println!("config {:?}", s);

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}

// Lazy static variable to hold the configuration
static SETTINGS: Lazy<Arc<Settings>> = Lazy::new(|| {
    let settings = Settings::new().expect("Failed to load configuration");
    Arc::new(settings)
});

// Function to access the configuration
pub fn settings() -> &'static Arc<Settings> {
    &SETTINGS
}
