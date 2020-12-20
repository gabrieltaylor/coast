// This is an example of middleware that keeps its own state and could
// be provided as a third party crate
#[derive(Default)]
struct RequestCounterMiddleware {
    requests_counted: Arc<AtomicUsize>,
}

impl RequestCounterMiddleware {
    fn new(start: usize) -> Self {
        Self {
            requests_counted: Arc::new(AtomicUsize::new(start)),
        }
    }
}

struct RequestCount(usize);

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for RequestCounterMiddleware {
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> Result {
        let count = self.requests_counted.fetch_add(1, Ordering::Relaxed);
        tide::log::trace!("request counter", { count: count });
        req.set_ext(RequestCount(count));

        let mut res = next.run(req).await;

        res.insert_header("request-number", count.to_string());
        Ok(res)
    }
}
