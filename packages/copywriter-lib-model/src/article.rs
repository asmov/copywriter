use asmov_copywriter_lib::{Slug, ModelBase};
use serde;
use garde;
use sqlx;
use crate::modeling::prelude::*;
use crate::*;

#[derive(Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, garde::Validate, sqlx::FromRow)]
#[serde(default)]
pub struct Article {
    #[serde(flatten)]
    #[sqlx(flatten)]
    #[garde(dive)]
    pub model_base: ModelBase,
    #[garde(dive)]
    pub author_slug: Slug,
    #[garde(length(min = 1))]
    pub published_timestamp: String,
}

impl Model for Article {
    fn model_base(&self) -> &ModelBase {
        &self.model_base
    }
}

impl ModelMut for Article {
    fn model_base_mut(&mut self) -> &mut ModelBase {
        &mut self.model_base
    }
}

impl ModelTypeAssoc for Article {
    const MODEL_NAME: &str = "Article";
    const MODEL_NAME_PLURAL: &str = "Articles";
    const MODEL_SLUG: &str = "article";
    const MODEL_SLUG_PLURAL: &str = "articles";
}

//impl ModelDeserializer for Article {}
impl Article {
    pub async fn db_query(pool: &sqlx::SqlitePool, slug: &str) -> anyhow::Result<Self> {
        let m: Self = sqlx::query_as("SELECT * FROM articles WHERE slug = ? LIMIT 1")
            .bind(slug)
            .fetch_one(pool).await?;

        Ok(m)
    }

    pub async fn db_insert(&self, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO articles (slug, name, subline, author_slug, published_timestamp) VALUES (?, ?, ?, ?, ?)")
        .bind(self.model_base.slug())
        .bind(self.model_base.name())
        .bind(self.model_base.subline())
        .bind(self.author_slug.as_str())
        .bind(&self.published_timestamp)
        .execute(pool).await?;

        Ok(())
    }

    pub async fn db_update(&self, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
        sqlx::query("UPDATE articles SET name = ?, subline = ?, author_slug = ?, published_timestamp = ? WHERE slug = ? LIMIT 1")
        .bind(self.model_base.name())
        .bind(self.model_base.subline())
        .bind(self.author_slug.as_str())
        .bind(&self.published_timestamp)
        .bind(self.model_base.slug())
        .execute(pool).await?;

        Ok(())
    }
}
