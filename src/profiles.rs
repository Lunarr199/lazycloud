use dirs;
use serde::Deserialize;
use std::{error::Error, fs, path::PathBuf};
use toml;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub name: String,
    pub from: String,
    pub to: String,
    pub mode: String,
    pub flags: Option<String>,
} // template of a sync profile

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sync: Vec<Profile>,
} // the config file that consists of multiple profiles

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = dirs::config_dir()
            .unwrap_or_default()
            .join("lazycloud/profiles.toml");

        let contents = fs::read_to_string(config_path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn path() -> PathBuf {
        let conf_dir = dirs::config_dir().unwrap_or_default().join("lazycloud");

        if conf_dir.exists() {
            return conf_dir;
        }

        fs::create_dir_all(&conf_dir).unwrap();
        conf_dir
    }
}
