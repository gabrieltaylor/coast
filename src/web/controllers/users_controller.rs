use crate::core::models::user::{NewUserChangeset, User};
use crate::web::views::users_view;
use crate::State;
use askama::Template;
use tide::http::mime;
use tide::{Redirect, Request, Response, StatusCode};
use uuid::Uuid;
use validator::Validate;

pub async fn index(req: Request<State>) -> tide::Result {
    match User::all(&req.state().pool).await {
        Ok(users) => {
            let res = Response::builder(200)
                .body(users_view::IndexTemplate::new(&users).render().unwrap())
                .content_type(mime::HTML)
                .build();
            Ok(res)
        }
        Err(e) => Ok(Response::new(StatusCode::Unauthorized)),
    }
}

pub async fn new(req: Request<State>) -> tide::Result {
    let res = Response::builder(200)
        .body(users_view::NewTemplate::new().render().unwrap())
        .content_type(mime::HTML)
        .build();
    Ok(res)
}

pub async fn show(req: Request<State>) -> tide::Result {
    let id = get_id_param(&req)?;

    match User::find(id, &req.state().pool).await {
        Ok(user) => {
            let res = Response::builder(200)
                .body(users_view::ShowTemplate::new(&user).render().unwrap())
                .content_type(mime::HTML)
                .build();
            Ok(res)
        }
        Err(e) => Ok(Response::new(StatusCode::Unauthorized)),
    }
}

pub async fn edit(req: Request<State>) -> tide::Result {
    let id = get_id_param(&req)?;

    match User::find(id, &req.state().pool).await {
        Ok(user) => {
            let res = Response::builder(200)
                .body(users_view::EditTemplate::new(&user).render().unwrap())
                .content_type(mime::HTML)
                .build();
            Ok(res)
        }
        Err(e) => Ok(Response::new(StatusCode::Unauthorized)),
    }
}

pub async fn create(mut req: Request<State>) -> tide::Result {
    let cs: NewUserChangeset = req.body_form().await?;

    match cs.validate() {
        Ok(_) => match User::create(cs, &req.state().pool).await {
            Ok(user) => {
                let res = Response::builder(200)
                    .body(users_view::ShowTemplate::new(&user).render().unwrap())
                    .content_type(mime::HTML)
                    .build();
                Ok(res)
            }
            Err(e) => Ok(Response::new(StatusCode::Unauthorized)),
        },
        Err(e) => Ok(Response::new(StatusCode::Unauthorized)),
    }
}

pub async fn update(mut req: Request<State>) -> tide::Result {
    let cs: NewUserChangeset = req.body_json().await?;

    match cs.validate() {
        Ok(_) => match User::create(cs, &req.state().pool).await {
            Ok(user) => {
                let res = Response::builder(200)
                    .body(users_view::ShowTemplate::new(&user).render().unwrap())
                    .content_type(mime::HTML)
                    .build();
                Ok(res)
            }
            Err(e) => Ok(Response::new(StatusCode::Unauthorized)),
        },
        Err(e) => Ok(Response::new(StatusCode::Unauthorized)),
    }
}

pub async fn delete(req: Request<State>) -> tide::Result {
    let id = get_id_param(&req)?;

    match User::delete(id, &req.state().pool).await {
        Ok(_) => Ok(Redirect::new("/users").into()),
        Err(e) => Ok(Response::new(StatusCode::Unauthorized)),
    }
}

fn get_id_param(req: &Request<State>) -> Result<Uuid, tide::Error> {
    let id = req.param("id");

    if id.is_err() {
        return Err(tide::Error::from_str(
            StatusCode::BadRequest,
            "No uuid specified",
        ));
    }

    match Uuid::parse_str(&id.unwrap()) {
        Err(_) => Err(tide::Error::from_str(
            StatusCode::BadRequest,
            "Invalid uuid specified",
        )),
        Ok(id) => Ok(id),
    }
}
