use sqlx::postgres::PgPool;
use tide::Result;

mod config;
mod core;
mod web;

use crate::config::CONFIG;

#[derive(Clone)]
pub struct State {
    pool: PgPool,
}

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::with_level(CONFIG.log.level);
    let pool = core::repo::connect().await?;
    let mut app = tide::with_state(State { pool: pool.clone() });

    web::middleware::attach(&mut app);
    web::router::attach(&mut app);

    app.listen(format!("{}:{}", CONFIG.server.host, CONFIG.server.port))
        .await?;

    println!(
        "Server started at {}:{} and ENV: {}",
        CONFIG.server.host, CONFIG.server.port, CONFIG.env
    );

    Ok(())
}
