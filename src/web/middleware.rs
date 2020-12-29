use crate::core::repo;
use tide::Server;

mod current_user;
mod now;

pub fn attach(app: &mut Server<repo::UserDatabase>) {
    app.with(now::call);
    app.with(current_user::call);
}
