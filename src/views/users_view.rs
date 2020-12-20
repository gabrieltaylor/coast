use askama::Template;
use tide::Body;

#[derive(Template)]
#[template(path = "users/index.html")]
pub struct IndexTemplate<'a> {
    name: &'a str,
}

impl<'a> IndexTemplate<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}
