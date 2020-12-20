#[derive(Clone, Default, Debug)]
pub struct UserDatabase;
impl UserDatabase {
    pub async fn find_user(&self) -> Option<User> {
        Some(User {
            name: "nori".into(),
        })
    }
}

#[derive(Debug)]
pub struct User {
    pub name: String,
}
