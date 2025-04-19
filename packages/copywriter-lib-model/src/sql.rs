use sqlx;
use anyhow;

pub async fn migrate_db(pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    sqlx::migrate!("db/migrations")
        .run(pool)
        .await?;

    Ok(())
}
