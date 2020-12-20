use tide::{Body, Request, Response, Server, StatusCode};
use crate::database;
use crate::views::users_view;
use std::convert::TryInto;
use askama::Template;

pub async fn index(req: Request<database::UserDatabase>) -> tide::Result {
    let user: &database::User = req.ext().unwrap();

    let mut res = Response::builder(200)
        .body(users_view::IndexTemplate::new(&user.name).render().unwrap())
        .build();
    Ok(res)
}

