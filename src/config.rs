use std::fs;
use std::path::{PathBuf};
use std::io::{self};
use dirs;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const CONFIG_FILE: &str = "rsnote.toml";
const DEFAULT_NOTES_DIR: &str = "rsnotes_storage";

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Config parse error: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("Config serialize error: {0}")]
    Serialize(#[from] toml::ser::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub notes_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let notes_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(DEFAULT_NOTES_DIR);
        Self { notes_dir }
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Self::init_config();
        }

        let config_str = fs::read_to_string(config_path)?;
        Ok(toml::from_str(&config_str)?)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let config_path = Self::config_path()?;
        let config_str = toml::to_string(self)?;

        fs::create_dir_all(config_path.parent().unwrap())?;
        fs::write(config_path, config_str)?;

        Ok(())
    }

    fn config_path() -> Result<PathBuf, ConfigError> {
        let config_dir = dirs::config_dir()
            .ok_or(ConfigError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                "Config directory not found",
            )))?;
        Ok(config_dir.join(CONFIG_FILE))
    }

    fn init_config() -> Result<Self, ConfigError> {
        let default_config = Self::default();

        // 交互式询问用户
        println!("Welcome to rsnote!");
        println!("Where would you like to store your notes?");
        println!("Default location: {}", default_config.notes_dir.display());
        println!("Press Enter to use default, or enter a custom path:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let notes_dir = if input.trim().is_empty() {
            default_config.notes_dir.clone()
        } else {
            PathBuf::from(input.trim())
        };

        // 创建配置
        let config = Config { notes_dir };
        config.save()?;

        // 创建笔记目录
        fs::create_dir_all(&config.notes_dir)?;

        println!("Configuration saved. Notes will be stored in: {}", config.notes_dir.display());
        Ok(config)
    }

}
