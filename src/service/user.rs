use crate::domain::user::{User, UserRepo, UserTrait};
use std::rc::Rc;

pub struct UserService<'a> {
    user_repo: Rc<dyn UserRepo + 'a>,
}

impl<'a> UserService<'a> {
    pub fn new(user: Rc<dyn UserRepo + 'a>) -> UserService<'a> {
        UserService { user_repo: user }
    }
}

impl UserTrait for UserService<'_> {
    fn create_user(&self, data: &User) {
        self.user_repo.create_user(data);
    }
}
