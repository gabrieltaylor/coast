use crate::State;
use tide::security::{CorsMiddleware, Origin};
use tide::Server;

mod now;

pub fn attach(app: &mut Server<State>) {
    app.with(now::call);

    let cors = CorsMiddleware::new()
        .allow_methods(
            "GET, POST, PUT, OPTIONS"
                .parse::<tide::http::headers::HeaderValue>()
                .unwrap(),
        )
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    app.with(cors);
}
