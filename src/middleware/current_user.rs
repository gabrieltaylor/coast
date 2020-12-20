use crate::database;
use std::future::Future;
use std::pin::Pin;
use tide::{Next, Request, Response, Result, StatusCode};

pub fn run<'a>(
    mut request: Request<database::UserDatabase>,
    next: Next<'a, database::UserDatabase>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
    Box::pin(async {
        if let Some(user) = request.state().find_user().await {
            tide::log::trace!("user loaded", {user: user.name});
            request.set_ext(user);
            Ok(next.run(request).await)
        // this middleware only needs to run before the endpoint, so
        // it just passes through the result of Next
        } else {
            // do not run endpoints, we could not find a user
            Ok(Response::new(StatusCode::Unauthorized))
        }
    })
}
