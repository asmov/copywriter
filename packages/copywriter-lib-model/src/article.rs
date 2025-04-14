use serde;
use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Article {
    pub headline: String,
    pub subheadline: String,    
    pub author_slug: String,
    pub published_timestamp: String,
}

impl Model for Article {
    const MODEL_NAME: "Article".to_string();
    const MODEL_SLUG: "article".to_string();
}
