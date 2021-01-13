use crate::web::controllers;
use crate::State;
use tide::Server;

pub fn attach(app: &mut Server<State>) {
    app.at("/users").get(controllers::users_controller::index);
    app.at("/users").post(controllers::users_controller::create);
    app.at("/users/new").get(controllers::users_controller::new);
    app.at("/users/:id")
        .get(controllers::users_controller::show);
    app.at("/users/:id/edit")
        .get(controllers::users_controller::edit);
    app.at("/users/:id")
        .post(controllers::users_controller::update);
    app.at("/users/:id")
        .patch(controllers::users_controller::update);
    app.at("/users/:id")
        .delete(controllers::users_controller::delete);

    app.at("/static")
        .serve_dir("static/")
        .expect("static directory does not exist in project root.");
}
