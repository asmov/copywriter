use asmov_copywriter_lib::{ModelBase, ModelMut};
use serde;
use garde;
use sqlx;
use crate::modeling::prelude::*;
use crate::*;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, garde::Validate, sqlx::FromRow)]
#[serde(default)]
pub struct Article {
    #[serde(flatten)]
    #[sqlx(flatten)]
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
impl Article {
    const SQL_SCHEMA: &str = r#"
        CREATE TABLE IF NOT EXISTS articles (
            slug TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            subline TEXT NOT NULL,
            author_slug TEXT NOT NULL,
            published_timestamp TEXT NOT NULL,
        );
    "#;

    pub fn sql_schema() -> &'static str {
        Self::SQL_SCHEMA
    }
}
