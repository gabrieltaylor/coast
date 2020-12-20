use crate::database;
use std::future::Future;
use std::pin::Pin;
use tide::{Next, Request, Result};

pub fn run<'a>(
    mut request: Request<database::UserDatabase>,
    next: Next<'a, database::UserDatabase>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
    Box::pin(async {
        request.set_ext(std::time::Instant::now());
        Ok(next.run(request).await)
    })
}
