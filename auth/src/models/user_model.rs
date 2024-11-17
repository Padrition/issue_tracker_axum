#[derive(Clone)]
pub struct CurrentUser{
    pub email: String,
    pub login: String,
    pub password_hash: String,
}