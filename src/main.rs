use tide::{Result, log, Server};
#[macro_use]
extern crate lazy_static;

mod config;
mod controllers;
mod database;
mod middleware;
mod router;
mod views;

lazy_static! {
    static ref CONFIG: config::Config =
        config::Config::new().expect("config can be loaded");
}

// trait Attachable {
//     fn attach(attachment: fn(&mut Self)) {
//         attachment.attach(Self)
//     }
// }
//
// impl Attachable for Server<database::UserDatabase> {
//     pub fn attach(attachment: fn(&mut Self)) {
//         attachment.attach(Self)
//     }
// }

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::with_level(log::LevelFilter::Debug);
    let mut app = tide::with_state(database::UserDatabase::default());

    middleware::attach(&mut app);
    router::attach(&mut app);

    println!(
        "Server started at {}:{} and ENV: {}",
        CONFIG.server.host, CONFIG.server.port, CONFIG.env
    );

    app.listen(format!("{}:{}", CONFIG.server.host, CONFIG.server.port)).await?;
    Ok(())
}
