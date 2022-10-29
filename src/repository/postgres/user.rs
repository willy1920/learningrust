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
        let result = sqlx::query_as!(
            User,
            r"INSERT INTO userr(name, email, password,id) VALUES($1,$2,$3,$4)",
            &data.name,
            data.email.as_ref(),
            &data.password,
            &data.id
        )
        .execute(&self.con)
        .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(CustomError::SqlxError(e))),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<User>, Error> {
        let result = sqlx::query_as!(User, r"SELECT id,name,email,password FROM userr")
            .fetch_all(&self.con)
            .await;
        match result {
            Ok(data) => Ok(data),
            Err(e) => Err(Error::new(CustomError::SqlxError(e))),
        }
    }

    pub async fn delete_user(&self, id: &i64) -> Result<(), Error> {
        let result = sqlx::query_as!(User, r"DELETE FROM userr WHERE id=$1", &id)
            .execute(&self.con)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(CustomError::SqlxError(e))),
        }
    }
}
