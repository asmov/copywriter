use sqlx;

pub async fn connect_db() -> anyhow::Result<sqlx::SqlitePool> {
    let pool = sqlx::Pool::<sqlx::Sqlite>::connect("sqlite::memory:").await?;

    /*let options = sqlx::sqlite::SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename("/tmp/copywriter.db");
    let pool = sqlx::SqlitePool::connect_with(options).await?;*/
    Ok(pool)
}
