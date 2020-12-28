use crate::controllers;
use crate::database;
use tide::Server;

pub fn attach(app: &mut Server<database::UserDatabase>) {
    app.at("/").get(controllers::users_controller::index);
}
