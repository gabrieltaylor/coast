use crate::State;
use std::future::Future;
use std::pin::Pin;
use tide::{Next, Request, Result};

pub fn call<'a>(
    mut request: Request<State>,
    next: Next<'a, State>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
    Box::pin(async {
        request.set_ext(std::time::Instant::now());
        Ok(next.run(request).await)
    })
}
