use crate::core::repo;
use std::future::Future;
use std::pin::Pin;
use tide::{Next, Request, Result};

pub fn call<'a>(
    mut request: Request<repo::UserDatabase>,
    next: Next<'a, repo::UserDatabase>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
    Box::pin(async {
        request.set_ext(std::time::Instant::now());
        Ok(next.run(request).await)
    })
}
