use crate::core::repo;
use crate::web::views::users_view;
use askama::Template;
use tide::{Request, Response};

pub async fn index(req: Request<repo::UserDatabase>) -> tide::Result {
    let user: &repo::User = req.ext().unwrap();

    let res = Response::builder(200)
        .body(users_view::IndexTemplate::new(&user.name).render().unwrap())
        .build();
    Ok(res)
}
