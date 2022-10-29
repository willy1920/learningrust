use crate::{domain::user::User, repository::postgres::user::UserPg};

pub struct UserService {
    user_repo: UserPg,
}

impl UserService {
    pub fn new(user: UserPg) -> UserService {
        UserService { user_repo: user }
    }

    pub async fn create_user(&self, data: &User) {
        let result = self.user_repo.create_user(data).await;
        match result {
            Ok(_) => {}
            Err(e) => println!("{:?}", e.to_string()),
        }
    }

    pub async fn get_all(&self) {
        let result = self.user_repo.get_all().await;
        match result {
            Ok(data) => println!("{:?}", data),
            Err(e) => println!("{:?}", e.to_string()),
        }
    }
}
