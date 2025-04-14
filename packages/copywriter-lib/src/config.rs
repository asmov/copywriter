use std::path::{Path, PathBuf};
use serde;
use toml;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub name: String,
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub url: String, 
}

impl Config {
    pub fn read_toml<P: AsRef<Path>>(file: P) -> anyhow::Result<Self> {
        let toml = std::fs::read_to_string(file)?;
        let config = toml::from_str(&toml)?;
        Ok(config)
    }
}
