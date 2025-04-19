use sqlx;

pub async fn connect_db() -> anyhow::Result<sqlx::SqlitePool> {
    let pool = sqlx::Pool::<sqlx::Sqlite>::connect("sqlite::memory:").await?;

    /*let options = sqlx::sqlite::SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename("/tmp/copywriter.db");
    let pool = sqlx::SqlitePool::connect_with(options).await?;*/
    Ok(pool)
}

pub async fn dump_db(pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    const SQL: &'static str = r"BEGIN TRANSACTION;
        OUT /tmp/copywriter.sql;
        DUMP;
        COMMIT;";

    sqlx::query(SQL).execute(pool).await?;
    Ok(())
}
