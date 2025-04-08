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
    #[serde(default = "default_brightness")]
    pub brightness: u8,
    #[serde(default = "default_screen_timeout")]
    pub screen_timeout: u64,
    #[serde(default = "default_periodic_off")]
    pub enable_periodic_off: bool,
    #[serde(default = "default_periodic_on_duration_seconds")]
    pub periodic_on_duration: u64,
    #[serde(default = "default_periodic_off_duration_seconds")]
    pub periodic_off_duration: u64,
    #[serde(default = "default_refresh_interval_ms")]
    pub refresh_interval_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FanConfig {
    pub temp_on: f32,
    pub temp_off: f32,
}

fn default_brightness() -> u8 {
    2
}
fn default_screen_timeout() -> u64 {
    300
}
fn default_periodic_off() -> bool {
    false
}
fn default_periodic_on_duration_seconds() -> u64 {
    10
}
fn default_periodic_off_duration_seconds() -> u64 {
    20
}
fn default_refresh_interval_ms() -> u64 {
    1000
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

    pub fn periodic_on_duration(&self) -> Duration {
        Duration::from_secs(self.display.periodic_on_duration)
    }

    pub fn periodic_off_duration(&self) -> Duration {
        Duration::from_secs(self.display.periodic_off_duration)
    }

    pub fn refresh_interval(&self) -> Duration {
        Duration::from_millis(self.display.refresh_interval_ms)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            display: DisplayConfig {
                brightness: default_brightness(),
                screen_timeout: default_screen_timeout(),
                enable_periodic_off: default_periodic_off(),
                periodic_on_duration: default_periodic_on_duration_seconds(),
                periodic_off_duration: default_periodic_off_duration_seconds(),
                refresh_interval_ms: default_refresh_interval_ms(),
            },
            fan: FanConfig {
                temp_on: 60.0,
                temp_off: 50.0,
            },
        }
    }
}
