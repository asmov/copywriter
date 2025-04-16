use serde;
use garde;
use crate::*;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, garde::Validate)]
#[serde(default)]
pub struct Article {
    #[garde(custom(valid_slug))]
    pub slug: String,
    #[garde(length(min = 1))]
    pub headline: String,
    #[garde(length(min = 1))]
    pub subheadline: String,
    #[garde(length(min = 1))]
    pub author_slug: String,
    #[garde(length(min = 1))]
    pub published_timestamp: String,
}

impl Model for Article {
    fn slug(&self) -> &str {
        &self.slug
    }
}

impl ModelTypeAssoc for Article {
    const MODEL_NAME: &str = "Article";
    const MODEL_NAME_PLURAL: &str = "Articles";
    const MODEL_SLUG: &str = "article";
    const MODEL_SLUG_PLURAL: &str = "articles";
}

impl ModelDeserializer for Article {}
