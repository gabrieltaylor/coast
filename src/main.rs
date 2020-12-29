use tide::Result;
#[macro_use]
extern crate lazy_static;

mod config;
mod core;
mod web;

lazy_static! {
    static ref CONFIG: config::Config =
        config::Config::new().expect("config can be loaded");
}

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::with_level(CONFIG.log.level);
    let mut app = tide::with_state(core::repo::UserDatabase::default());

    web::middleware::attach(&mut app);
    web::router::attach(&mut app);

    println!(
        "Server started at {}:{} and ENV: {}",
        CONFIG.server.host, CONFIG.server.port, CONFIG.env
    );

    app.listen(format!("{}:{}", CONFIG.server.host, CONFIG.server.port)).await?;
    Ok(())
}
