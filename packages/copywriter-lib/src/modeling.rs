
pub trait Model {
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

    fn model_json_filename(&self) -> String {
        format!("{}.json", Self::MODEL_SLUG)
    }

    fn json_slug_filename(&self) -> String {
        format!("{}.json", self.slug())
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

    fn type_json_filename() -> String {
        format!("{}.json", Self::MODEL_SLUG)
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
