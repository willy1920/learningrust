use crate::domain::{
    error::{CustomError, Error},
    user::User,
};
use sqlx::{Pool, Postgres};

pub struct UserPg {
    con: Pool<Postgres>,
}

impl UserPg {
    pub fn new(pool: Pool<Postgres>) -> Self {
        UserPg { con: pool }
    }

    pub async fn create_user(&self, data: &User) -> Result<(), Error> {
        let result = sqlx::query("INSERT INTO MAIL.USER(name, email, password) VALUES($1,$2,$3)")
            .bind(&data.name)
            .bind(&data.email)
            .bind(&data.password)
            .execute(&self.con)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(CustomError::SqlxError(e))),
        }
    }
}
