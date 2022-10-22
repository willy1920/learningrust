#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub trait UserRepo {
    fn create_user(&self, data: &User);
}

pub trait UserTrait {
    fn create_user(&self, data: &User);
}
