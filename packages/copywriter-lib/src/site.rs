use std::path::{Path, PathBuf};
use serde;
use toml;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Site {
    pub name: String,
    pub url: String,
    pub owner: String,
    pub owner_url: String,
    pub description: String,
}

impl Site {
    pub const TOML_FILENAME: &'static str = "site.toml";

    pub fn read_toml<P: AsRef<Path>>(file: P) -> anyhow::Result<Self> {
        let toml = std::fs::read_to_string(file)?;
        let config = toml::from_str(&toml)?;
        Ok(config)
    }
}

