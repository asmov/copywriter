use asmov_copywriter_lib::{Slug, ModelCore};
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
    pub model_meta: ModelMeta,
    #[serde(flatten)]
    #[sqlx(flatten)]
    #[garde(dive)]
    pub model_core: ModelCore,
    #[garde(dive)]
    pub author_slug: Slug,
}

impl Article {
    pub fn author_slug(&self) -> &Slug {
        &self.author_slug
    }
}

impl Model for Article {
    fn model_meta(&self) -> &ModelMeta {
        &self.model_meta
    }

    fn model_core(&self) -> &ModelCore {
        &self.model_core
    }
}

impl ModelMut for Article {
    fn model_meta_mut(&mut self) -> &mut ModelMeta {
        &mut self.model_meta
    }

    fn model_core_mut(&mut self) -> &mut ModelCore {
        &mut self.model_core
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

    pub async fn db_upsert(&self, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
        sqlx::query(r"
            INSERT INTO articles (
                created_time,
                modified_time,
                slug,
                name,
                subline,
                author_slug) VALUES (?,?,?,?,?,?)
            ON CONFLICT (slug) DO UPDATE SET
                modified_time = excluded.modified_time,
                subline = excluded.subline,
                author_slug = excluded.author_slug
        ")
        .bind(self.created_time())
        .bind(self.modified_time())
        .bind(self.slug())
        .bind(self.name())
        .bind(self.subline())
        .bind(self.author_slug())
        .execute(pool).await?;

        Ok(())
    }

    pub async fn db_update(&self, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
        sqlx::query(r"UPDATE articles
            SET created_time = ?,
                modified_time = ?,
                name = ?,
                subline = ?,
                author_slug = ?
            WHERE slug = ?
            LIMIT 1")
        .bind(self.created_time())
        .bind(self.modified_time())
        .bind(self.name())
        .bind(self.subline())
        .bind(self.author_slug().as_str())
        .bind(self.slug())
        .execute(pool).await?;

        Ok(())
    }
}
