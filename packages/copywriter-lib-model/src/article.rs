use serde;
use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Article {
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
