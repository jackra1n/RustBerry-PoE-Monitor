use log::{debug, info, warn};
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub display: DisplayConfig,
    pub fan: FanConfig,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

const DEFAULT_CONFIG: &str = include_str!("../config.toml");

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;

        if let Some(parent_dir) = config_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        let config_str = if config_path.exists() {
            debug!("Loading config file from: {:?}", config_path);
            fs::read_to_string(&config_path)?
        } else {
            fs::write(&config_path, DEFAULT_CONFIG)?;
            info!("Created default config file at: {:?}", config_path);
            DEFAULT_CONFIG.to_string()
        };

        let config: Config = toml::from_str(&config_str)?;
        config.validate()?;
        Ok(config)
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

    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.display.brightness > 4 {
            return Err(format!(
                "Invalid brightness value: {}. Must be between 0 and 4",
                self.display.brightness
            )
            .into());
        }
        if self.fan.temp_off <= 0.0 || self.fan.temp_on <= 0.0 {
            return Err("Temperatures must be greater than 0".into());
        }
        if self.fan.temp_on <= self.fan.temp_off {
            return Err("temp_on must be greater than temp_off".into());
        }
        Ok(())
    }
}
