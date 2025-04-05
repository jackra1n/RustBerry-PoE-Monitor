use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub display: DisplayConfig,
    pub fan: FanConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayConfig {
    pub brightness: u8,
    pub screen_timeout: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FanConfig {
    pub temp_on: f32,
    pub temp_off: f32,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;

        if let Some(parent_dir) = config_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        if !config_path.exists() {
            let default_config = Config::default();
            let toml_string = toml::to_string_pretty(&default_config)?;
            fs::write(&config_path, toml_string)?;
            info!("Created default config file at: {:?}", config_path);
            Ok(default_config)
        } else {
            debug!("Loading config file from: {:?}", config_path);
            let config_str = fs::read_to_string(config_path)?;
            let config: Config = toml::from_str(&config_str)?;
            Ok(config)
        }
    }

    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        match env::var("HOME") {
            Ok(home_dir) => {
                let mut path = PathBuf::from(home_dir);
                path.push(".config");
                path.push("rustberry-poe-monitor");
                path.push("config.toml");
                Ok(path)
            }
            Err(_) => {
                warn!("HOME environment variable not set. Using config.toml in current directory.");
                Ok(PathBuf::from("config.toml"))
            }
        }
    }

    pub fn display_timeout(&self) -> Duration {
        Duration::from_secs(self.display.screen_timeout)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            display: DisplayConfig {
                brightness: 2,
                screen_timeout: 300,
            },
            fan: FanConfig {
                temp_on: 60.0,
                temp_off: 50.0,
            },
        }
    }
}
