use crate::database;
use crate::views::users_view;
use askama::Template;
use tide::{Request, Response};

pub async fn index(req: Request<database::UserDatabase>) -> tide::Result {
    let user: &database::User = req.ext().unwrap();

    let res = Response::builder(200)
        .body(users_view::IndexTemplate::new(&user.name).render().unwrap())
        .build();
    Ok(res)
}
