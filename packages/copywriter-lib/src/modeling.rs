use serde;
use garde;

pub trait Model:
    std::fmt::Debug
    + Default
    + garde::Validate
{
    fn slug(&self) -> &str;
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

pub trait ModelTypeAssoc: Model + ModelDeserializer {
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
        &Self::MODEL_SLUG_PLURAL
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
        &Self::MODEL_SLUG_PLURAL
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

pub fn valid_slug(value: &String, _context: &()) -> garde::Result {
    if value.is_empty() {
        Err(garde::Error::new("empty slug"))
    } else if value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        Ok(())
    } else {
        Err(garde::Error::new("invalid slug"))
    }
}
