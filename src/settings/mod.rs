mod database;
mod casbin;

use std::env;

use config::{Config, Environment, File};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use self::{casbin::CasbinConfig, database::DatabaseConfig};

pub static APP_SETTINGS: Lazy<Settings> = Lazy::new(init_app_settings);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// App and server configuration
pub struct AppConfig {
  /// Server's port
  pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisConfig {
  /// Redis connection URI
  pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggerConfig {
  /// What should the (terminal) logger print
  pub level: String,
  /// File logger path output
  pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
  /// App and server configuration
  /// pub app: AppConfig,
  /// Casbin configuration
  pub casbin: CasbinConfig,
  /// SQLx database configuration
  pub database: DatabaseConfig,
}

fn init_app_settings() -> Settings {
    // Start config
    let mut s = Config::default();

    // Create a path
    let mut config_file_path = env::current_dir().expect("Cannot get current path");

    // Get current RUN_MODE, should be: development/production
    let current_env = env::var("RUN_MODE").unwrap_or_else(|_| String::from("development"));

    // From current path add /environments
    config_file_path.push("environments");
    // Add RUN_MODE.yaml
    config_file_path.push(format!("{}.yaml", current_env));

    // Add in the current environment file
    // Default to 'development' env
    s.merge(File::from(config_file_path).required(false))
      .expect("Could not read file");

    // Add in settings from the environment
    // ex. APP_DEBUG=1 sets debug key, APP_DATABASE_URL sets database.url key
    s.merge(Environment::new().prefix("APP").separator("_"))
      .expect("Cannot get env");

    // Deserialize configuration
    s.try_into().expect("Configuration error")
}
