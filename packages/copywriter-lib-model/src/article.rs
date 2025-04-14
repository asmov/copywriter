use serde;
use validator;
use crate::*;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, validator::Validate)]
#[serde(default)]
pub struct Article {
    #[validate(custom(function = validate_slug))]
    pub slug: String,
    pub headline: String,
    pub subheadline: String,
    pub author_slug: String,
    pub published_timestamp: String,
}

impl ModelType for Article {
    const MODEL_NAME: &str = "Article";
    const MODEL_SLUG: &str = "article";
}

impl Model for Article {
    fn slug(&self) -> &str {
        &self.slug
    }
}
