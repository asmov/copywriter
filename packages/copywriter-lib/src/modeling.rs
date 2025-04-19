use std::ops::{Deref, DerefMut};
use slugify::slugify;

use serde;
use garde;
use validation::*;

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize, garde::Validate, sqlx::Type)]
#[sqlx(transparent)]
pub struct Timestamp(
    #[garde(skip)]
    #[sqlx()]
    chrono::DateTime<chrono::Utc>);

impl Timestamp {
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, nanosecond: u32) -> Option<Timestamp> {
        Some(Timestamp(chrono::NaiveDate::from_ymd_opt(year, month, day)?
            .and_hms_nano_opt(hour, minute, second, nanosecond)?
            .and_utc()))
    }

    pub fn now() -> Timestamp {
        Timestamp(chrono::Utc::now())
    }

    pub fn datetime(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.0
    }
}

pub mod prelude {
    pub use super::{Model, ModelMut, ModelMeta, ModelCore, Timestamp, ModelTypeAssoc, validation::*, garde::Validate};
}

#[derive(Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, garde::Validate, sqlx::Type)]
#[sqlx(transparent)]
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

#[derive(Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, garde::Validate, sqlx::FromRow)]
#[serde(default)]
pub struct ModelMeta {
    #[garde(dive)]
    pub created_time: Timestamp,
    #[garde(dive)]
    pub modified_time: Timestamp
}

impl ModelMeta {
    pub fn created_time(&self) -> &Timestamp {
        &self.created_time
    }

    pub fn modified_time(&self) -> &Timestamp {
        &self.modified_time
    }

    pub fn set_created_time(&mut self, created_time: Timestamp) {
        self.created_time = created_time;
    }

    pub fn set_modified_time(&mut self, modified_time: Timestamp) {
        self.modified_time = modified_time;
    }
}

#[derive(Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, garde::Validate, sqlx::FromRow)]
#[serde(default)]
pub struct ModelCore {
    #[garde(dive)]
    pub slug: Slug,
    #[garde(length(min = 1, max = 255))]
    pub name: String,
    #[garde(length(min = 1, max = 255))]
    pub subline: String,
}

impl ModelCore {
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

    pub fn merge(mut self, other: ModelCore) -> Self {
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
    fn model_meta(&self) -> &ModelMeta;
    fn model_core(&self) -> &ModelCore;

    #[inline]
    fn created_time(&self) -> &Timestamp {
        &self.model_meta().created_time
    }

    #[inline]
    fn modified_time(&self) -> &Timestamp {
        &self.model_meta().modified_time
    }

    /// The unique identifier for data in this model. Must match the model's
    /// name when passed through [slugify::slugify].
    #[inline]
    fn slug(&self) -> &str {
        self.model_core().slug()
    }

    /// Used in headers, titles, etc.
    #[inline]
    fn name(&self) -> &str {
        self.model_core().name()
    }

    /// Used in subheadlines, very short descriptions for lists, etc.
    #[inline]
    fn subline(&self) -> &str {
        self.model_core().subline()
    }
}

pub trait ModelMut: Model {
    fn model_meta_mut(&mut self) -> &mut ModelMeta;
    fn model_core_mut(&mut self) -> &mut ModelCore;

    fn set_created_time(&mut self, created_time: Timestamp) {
        self.model_meta_mut().set_created_time(created_time);
    }

    fn set_modified_time(&mut self, modified_time: Timestamp) {
        self.model_meta_mut().set_modified_time(modified_time);
    }

    fn set_name(&mut self, name: String) {
        self.model_core_mut().set_name(name);
    }

    fn set_slug(&mut self, slug: Slug) {
        self.model_core_mut().set_slug(slug);
    }

    fn set_subline(&mut self, subline: String) {
        self.model_core_mut().set_subline(subline);
    }

    fn merge_base(mut self, other: ModelCore) -> Self {
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
    pub model: M,
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
