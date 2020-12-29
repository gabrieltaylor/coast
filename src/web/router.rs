use crate::web::controllers;
use crate::core::repo;
use tide::Server;

pub fn attach(app: &mut Server<repo::UserDatabase>) {
    app.at("/").get(controllers::users_controller::index);
}
