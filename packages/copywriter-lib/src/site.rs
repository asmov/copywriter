use std::path::Path;
use anyhow::Context;
use serde;
use garde;
use toml;

use crate::{modeling::prelude::*, ModelBase};


#[derive(Debug, Default, serde::Serialize, serde::Deserialize, garde::Validate)]
#[serde(default)]
pub struct Site {
    #[serde(flatten)]
    #[garde(dive)]
    pub model_base: ModelBase,
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
    fn model_base(&self) -> &ModelBase {
        &self.model_base
    }
}

impl ModelMut for Site {
    fn model_base_mut(&mut self) -> &mut ModelBase {
        &mut self.model_base
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
