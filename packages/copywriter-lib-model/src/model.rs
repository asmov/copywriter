
pub trait Model {
    const MODEL_NAME: String;    
    const MODEL_SLUG: String;

    fn model_name(&self) -> &str {
        &Self::MODEL_NAME
    }

    fn model_slug(&self) -> &str {
        &Self::MODEL_SLUG
    }
}

#[derive(Debug, Copy, PartialEq, Eq)]
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

