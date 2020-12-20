use tide::{Request, Response, Server, StatusCode};
use crate::database;
use crate::controllers;

pub fn attach(app: &mut Server<database::UserDatabase>) {
    app.at("/").get(controllers::users_controller::index);
}
