use tide::{Request, Response, Server, StatusCode};
use tide::utils::{After, Before};
use tide::http::mime;
use crate::database;

mod current_user;
mod now;
// mod fallback;
// mod request_counter;

pub fn attach(app: &mut Server<database::UserDatabase>) {
    // app.with(middleware::fallback);
    // app.with(middleware::request_counter); //RequestCounterMiddleware::new(0));
    // app.with(fallback::run);
    app.with(now::run);
    app.with(current_user::run);
}
