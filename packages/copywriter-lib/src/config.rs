use std::path::{Path, PathBuf};
use serde;
use toml;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl Config {
    pub const TOML_FILENAME: &'static str = "copywriter.toml";

    pub fn read_toml<P: AsRef<Path>>(file: P) -> anyhow::Result<Self> {
        let toml = std::fs::read_to_string(&file)?;
        let mut config: Self = toml::from_str(&toml)?;
        let config_dir = file.as_ref().parent().unwrap();
        config.input_dir = config_dir.join(config.input_dir);
        config.output_dir = config_dir.join(config.output_dir);
        Ok(config)
    }
}
