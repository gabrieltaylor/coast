use std::future::Future;
use std::pin::Pin;
use tide::{Next, Request, Response, Result, StatusCode};
use crate::State;
use crate::core::models::user::User;
use uuid::Uuid;

pub fn call<'a>(
    mut req: Request<State>,
    next: Next<'a, State>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
    Box::pin(async {
        match User::find(Uuid::new_v4(), &req.state().pool).await {
            Err(_) => {
                Ok(Response::new(StatusCode::Unauthorized))
            },
            Ok(user) => {
                tide::log::trace!("user loaded", {user: user.name});
                req.set_ext(user);
                Ok(next.run(req).await)
            }
    }})
}
