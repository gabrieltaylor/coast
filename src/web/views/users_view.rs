use crate::core::models::user::User;
use askama::Template;

#[derive(Template)]
#[template(path = "users/index.html")]
pub struct IndexTemplate<'a> {
    users: &'a Vec<User>,
}

impl<'a> IndexTemplate<'a> {
    pub fn new(users: &'a Vec<User>) -> Self {
        Self { users }
    }
}

#[derive(Template)]
#[template(path = "users/new.html")]
pub struct NewTemplate {}

impl NewTemplate {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Template)]
#[template(path = "users/show.html")]
pub struct ShowTemplate<'a> {
    user: &'a User,
}

impl<'a> ShowTemplate<'a> {
    pub fn new(user: &'a User) -> Self {
        Self { user }
    }
}

#[derive(Template)]
#[template(path = "users/edit.html")]
pub struct EditTemplate<'a> {
    user: &'a User,
}

impl<'a> EditTemplate<'a> {
    pub fn new(user: &'a User) -> Self {
        Self { user }
    }
}
