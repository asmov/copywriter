use std::path::Path;
use anyhow::Context;
use serde;
use garde;
use toml;

use crate::{modeling::prelude::*, ModelCore};


#[derive(Debug, Default, serde::Serialize, serde::Deserialize, garde::Validate)]
#[serde(default)]
pub struct Site {
    #[serde(flatten)]
    #[garde(dive)]
    pub model_meta: ModelMeta,
    #[serde(flatten)]
    #[garde(dive)]
    pub model_core: ModelCore,
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
    fn model_meta(&self) -> &ModelMeta {
        &self.model_meta
    }

    fn model_core(&self) -> &ModelCore {
        &self.model_core
    }
}

impl ModelMut for Site {
    fn model_meta_mut(&mut self) -> &mut ModelMeta {
        &mut self.model_meta
    }

    fn model_core_mut(&mut self) -> &mut ModelCore {
        &mut self.model_core
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
