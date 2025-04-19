use sqlx;

pub async fn connect_db() -> anyhow::Result<sqlx::SqlitePool> {
    let pool = sqlx::Pool::<sqlx::Sqlite>::connect("sqlite::memory:").await?;
    Ok(pool)
}
