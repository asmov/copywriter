use sqlx;
use tokio;
use tokio::runtime::Handle;


pub fn connect_db() -> anyhow::Result<sqlx::SqlitePool> {
    let pool = tokio::task::block_in_place(|| {
        Handle::current().block_on(async move {
            sqlx::Pool::<sqlx::Sqlite>::connect("sqlite::memory:").await
        })
    })?;

    Ok(pool)
}
