use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_file: Option<PathBuf>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub realms: Vec<String>,
}

impl Config {
    pub fn path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("could not determine config directory")?
            .join("1seed");
        Ok(config_dir.join("config.toml"))
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::path()?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;

        Ok(())
    }

    pub fn add_realm(&mut self, name: &str) {
        if !self.realms.contains(&name.to_string()) {
            self.realms.push(name.to_string());
            self.realms.sort();
        }
    }

    pub fn remove_realm(&mut self, name: &str) {
        self.realms.retain(|r| r != name);
    }
}
