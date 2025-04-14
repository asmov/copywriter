use serde;
use validator;

pub trait Model:
    std::fmt::Debug
    + Default
    + serde::Serialize
    + serde::de::DeserializeOwned
    + validator::Validate
{
    fn slug(&self) -> &str;
}

pub trait ModelType: Model {
    const MODEL_NAME: &'static str;
    const MODEL_SLUG: &'static str;

    fn model_name(&self) -> &'static str {
        &Self::MODEL_NAME
    }

    fn model_slug(&self) -> &'static str {
        &Self::MODEL_SLUG
    }

    fn model_dirname(&self) -> &'static str {
        &Self::MODEL_SLUG
    }
    fn model_toml_data_filename(&self) -> String {
        format!("{}.toml", Self::MODEL_SLUG)
    }

    fn model_json_data_filename(&self) -> String {
        format!("{}.json", Self::MODEL_SLUG)
    }

    fn model_markdown_content_filename(&self) -> String {
        format!("{}.md", Self::MODEL_SLUG)
    }

    fn toml_data_slug_filename(&self) -> String {
        format!("{}.toml", self.slug())
    }

    fn json_data_slug_filename(&self) -> String {
        format!("{}.json", self.slug())
    }

    fn markdown_content_slug_filename(&self) -> String {
        format!("{}.md", self.slug())
    }

    fn type_name() -> &'static str {
        &Self::MODEL_NAME
    }

    fn type_slug() -> &'static str {
        &Self::MODEL_SLUG
    }

    fn type_dirname() -> &'static str {
        &Self::MODEL_SLUG
    }

    fn type_toml_data_filename() -> String {
        format!("{}.toml", Self::MODEL_SLUG)
    }

    fn type_json_data_filename() -> String {
        format!("{}.json", Self::MODEL_SLUG)
    }

    fn type_markdown_content_filename() -> String {
        format!("{}.md", Self::MODEL_SLUG)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ContentKind {
    Html,
    Markdown
}

pub trait Content {
    const CONTENT_KIND: ContentKind;

    fn content(&self) -> &str;

    fn kind(&self) -> ContentKind {
        Self::CONTENT_KIND
    }
}

pub struct HtmlContent {
    pub content: String
}

impl Content for HtmlContent {
    const CONTENT_KIND: ContentKind = ContentKind::Html;

    fn content(&self) -> &str {
        &self.content
    }
}

pub struct MarkdownContent {
    pub content: String
}

impl Content for MarkdownContent {
    const CONTENT_KIND: ContentKind = ContentKind::Markdown;

    fn content(&self) -> &str {
        &self.content
    }
}

pub fn validate_slug(value: &String) -> Result<(), validator::ValidationError> {
    if value.is_empty() {
        Err(validator::ValidationError::new("empty slug"))
    } else if value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid slug"))
    }
}
