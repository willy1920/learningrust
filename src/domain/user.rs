#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub password: String,
}
