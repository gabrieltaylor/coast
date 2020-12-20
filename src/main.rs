use std::future::Future;
use tide::Result;

mod middleware;
mod database;
mod controllers;
mod routes;
mod views;

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::start();
    let mut app = tide::with_state(database::UserDatabase::default());

    middleware::attach(&mut app);
    routes::attach(&mut app);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
