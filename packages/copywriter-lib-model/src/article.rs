use asmov_copywriter_lib::{ModelBase, ModelMut};
use serde;
use garde;
use crate::modeling::prelude::*;
use crate::*;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, garde::Validate)]
#[serde(default)]
pub struct Article {
    #[serde(flatten)]
    #[garde(dive)]
    pub model_base: ModelBase,
    #[garde(custom(valid_slug))]
    pub author_slug: String,
    #[garde(length(min = 1))]
    pub published_timestamp: String,
}

impl Model for Article {
    fn model_base(&self) -> &ModelBase {
        &self.model_base
    }
}

impl ModelMut for Article {
    fn model_base_mut(&mut self) -> &mut ModelBase {
        &mut self.model_base
    }
}

impl ModelTypeAssoc for Article {
    const MODEL_NAME: &str = "Article";
    const MODEL_NAME_PLURAL: &str = "Articles";
    const MODEL_SLUG: &str = "article";
    const MODEL_SLUG_PLURAL: &str = "articles";
}

//impl ModelDeserializer for Article {}
