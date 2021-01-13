use crate::config::CONFIG;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn connect() -> tide::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .min_connections(CONFIG.database.pool_size)
        .max_connections(CONFIG.database.pool_size)
        .connect(&CONFIG.database.uri())
        .await?;
    Ok(pool)
}
