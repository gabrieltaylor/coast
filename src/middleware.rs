use crate::database;
use tide::Server;

mod current_user;
mod now;

pub fn attach(app: &mut Server<database::UserDatabase>) {
    app.with(now::run);
    app.with(current_user::run);
}
