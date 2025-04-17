use std::path::Path;
use anyhow::Context;
use serde;
use toml;

use crate::modeling::prelude::*;


#[derive(Debug, Default, serde::Serialize, serde::Deserialize, garde::Validate)]
pub struct Site {
    #[garde(custom(valid_slug))]
    pub slug: String,
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(length(min = 1))]
    pub subline: String,
    #[garde(length(min = 1))]
    pub url: String,
    #[garde(length(min = 1))]
    pub owner_name: String,
    #[garde(length(min = 1))]
    pub owner_url: String,
    #[garde(length(min = 1))]
    pub description: String,
}

impl Model for Site {
    fn slug(&self) -> &str {
        &self.slug
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn subline(&self) -> &str {
        &self.subline
    }
}

impl Site {
    pub const TOML_FILENAME: &'static str = "site.toml";

    pub fn read_toml<P: AsRef<Path>>(file: P) -> anyhow::Result<Self> {
        let toml = std::fs::read_to_string(&file)
            .with_context(|| format!("Unable to read site config: {}", file.as_ref().display()))?;
        let config = toml::from_str(&toml)?;
        Ok(config)
    }
}
