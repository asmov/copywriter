use std::path::{Path, PathBuf};
use serde;
use toml;
use anyhow::bail;
use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub config_dir: PathBuf,
    pub config: Config,
    pub site: Site
}

impl Project {
    pub fn locate() -> anyhow::Result<Self> {
        Self::locate_from(env::current_dir()?);
    }

    pub fn locate_from<P: AsRef<Path>>(dir: P) -> anyhow::Result<Self> {
        let mut dir = *dir;
        loop {
            let config_filepath =  dir.join(Config::TOML_FILENAME);
            if config_filepath.exists() {
                return Self::new(config_filepath)
            } else {
                match dir.parent() {
                    Ok(parent) => dir = parent;
                    Err(_) => bail!("Unable to locate config `{}`", Config::TOML_FILENAME)
                }
            }
        }
    }

    pub fn new(config_filepath: PathBuf) -> anyhow::Result<Self> {
        let config_dir = config_filepath.parent();
        let config = Config::read_toml(config_filepath)?;
        let site_filepath = config.input_dir().join(Site::TOML_FILENAME)?;
        let site = Site::read_toml(site_filepath)?;

        Ok(Self {
            config_dir,
            config, 
            site
        })
    }
}

