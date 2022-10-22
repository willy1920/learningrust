use sqlx::{Pool, Postgres};

use crate::domain::user::{User, UserRepo};

pub struct UserPg {
    con: Pool<Postgres>,
}

impl UserPg {
    pub fn new(pool: Pool<Postgres>) -> Self {
        UserPg { con: pool }
    }
}

impl UserRepo for UserPg {
    fn create_user(&self, data: &User) {
        println!("{:?}", data);
    }
}
