use sqlx::{Pool, Postgres};

pub struct UserPg {
    con: Pool<Postgres>,
}

impl UserPg {
    pub fn new(pool: Pool<Postgres>) -> Self {
        UserPg { con: pool }
    }
}
