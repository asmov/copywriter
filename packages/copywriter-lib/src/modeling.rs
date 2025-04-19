use std::ops::{Deref, DerefMut};
use slugify::slugify;

use serde;
use garde;
use validation::*;

pub mod prelude {
    pub use super::{Model, ModelMut, ModelBase, ModelTypeAssoc, validation::*, garde::Validate};
}

#[derive(Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, garde::Validate)]
#[serde(default)]
#[garde(transparent)]
pub struct Slug(
    #[garde(custom(valid_slug))]
    String
);

impl Deref for Slug {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Slug {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&String> for Slug {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for Slug {
    fn from(value: &str) -> Self {
        Slug(slugify::slugify!(value))
    }
}

#[derive(Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, garde::Validate)]
#[serde(default)]
pub struct ModelBase {
    #[garde(dive)]
    pub slug: Slug,
    #[garde(length(min = 1, max = 255))]
    pub name: String,
    #[garde(length(min = 1, max = 255))]
    pub subline: String,
}

impl ModelBase {
    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn subline(&self) -> &str {
        &self.subline
    }

    pub fn set_slug(&mut self, slug: Slug) {
        self.slug = slug;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_subline(&mut self, subline: String) {
        self.subline = subline;
    }

    pub fn merge(mut self, other: ModelBase) -> Self {
        if self.name.is_empty() {
            self.name = other.name;
        }
        if self.slug.is_empty() {
            self.slug = other.slug;
        }
        if self.subline.is_empty() {
            self.subline = other.subline;
        }

        self
    }
}

pub trait Model:
    std::fmt::Debug
    + Default
    + garde::Validate
{
    fn model_base(&self) -> &ModelBase;

    /// The unique identifier for data in this model. Must match the model's
    /// name when passed through [slugify::slugify].
    #[inline]
    fn slug(&self) -> &str {
        self.model_base().slug()
    }

    /// Used in headers, titles, etc.
    #[inline]
    fn name(&self) -> &str {
        self.model_base().name()
    }

    /// Used in subheadlines, very short descriptions for lists, etc.
    #[inline]
    fn subline(&self) -> &str {
        self.model_base().subline()
    }
}

pub trait ModelMut: Model {
    fn model_base_mut(&mut self) -> &mut ModelBase;

    fn set_name(&mut self, name: String) {
        self.model_base_mut().set_name(name);
    }

    fn set_slug(&mut self, slug: Slug) {
        self.model_base_mut().set_slug(slug);
    }

    fn set_subline(&mut self, subline: String) {
        self.model_base_mut().set_subline(subline);
    }

    fn merge_base(mut self, other: ModelBase) -> Self {
        if self.name().is_empty() {
            self.set_name(other.name);
        }
        if self.slug().is_empty() {
            self.set_slug(other.slug);
        }
        if self.subline().is_empty() {
            self.set_subline(other.subline);
        }

        self
    }
}

pub struct ModelTypeRegistry {
    model_types: Vec<ModelType>,
}

impl ModelTypeRegistry {
    pub fn new() -> Self {
        Self {
            model_types: Vec::new(),
        }
    }

    pub fn register(&mut self, model_type: ModelType) {
        self.model_types.push(model_type);
    }

    pub fn iter(&self) -> impl Iterator<Item = &ModelType> {
        self.model_types.iter()
    }
}

pub struct ModelType {
    name: String,
    name_plural: String,
    slug: String,
    slug_plural: String,
    //toml_deserializer: fn(&str) -> Result<impl Model, toml::de::Error>
}

impl ModelType {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_plural(&self) -> &str {
        &self.name_plural
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn slug_plural(&self) -> &str {
        &self.slug_plural
    }
}

pub trait ModelDeserializer {
    fn from_toml(toml: &str) -> Result<Self, toml::de::Error>
    where
        Self: serde::de::DeserializeOwned
    {
        let model: Self = toml::from_str(toml)?;
        Ok(model)
    }
}

pub trait ModelTypeAssoc: Model /*+ ModelDeserializer*/ {
    const MODEL_NAME: &'static str;
    const MODEL_NAME_PLURAL: &'static str;
    const MODEL_SLUG: &'static str;
    const MODEL_SLUG_PLURAL: &'static str;

    fn model_type() -> ModelType {
        ModelType {
            name: Self::MODEL_NAME.to_string(),
            name_plural: Self::MODEL_NAME_PLURAL.to_string(),
            slug: Self::MODEL_SLUG.to_string(),
            slug_plural: Self::MODEL_SLUG_PLURAL.to_string(),
            //toml_deserializer: Self::from_toml,
        }
    }

    fn model_name(&self) -> &'static str {
        &Self::MODEL_NAME
    }

    fn model_name_plural(&self) -> &'static str {
        &Self::MODEL_NAME_PLURAL
    }

    fn model_slug(&self) -> &'static str {
        &Self::MODEL_SLUG
    }

    fn model_slug_plural(&self) -> &'static str {
        &Self::MODEL_SLUG_PLURAL
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

    fn type_name_plural() -> &'static str {
        &Self::MODEL_NAME_PLURAL
    }

    fn type_slug() -> &'static str {
        &Self::MODEL_SLUG
    }

    fn type_slug_plural() -> &'static str {
        &Self::MODEL_SLUG_PLURAL
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ModelBundle<M: Model> {
    pub meta: M,
    pub content: Content
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Content {
    pub html: Option<String>,
    pub quote_html: Option<String>,
    pub markdown: Option<String>,
}

pub mod validation {
    pub fn valid_slug(value: &String, _context: &()) -> garde::Result {
        if value.is_empty() {
            Err(garde::Error::new("empty slug"))
        } else if value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            Ok(())
        } else {
            Err(garde::Error::new("invalid slug"))
        }
    }
}
