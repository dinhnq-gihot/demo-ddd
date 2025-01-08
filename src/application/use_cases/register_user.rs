use crate::domain::{
    entities::user::User, repositories::user_repository::UserRepository,
    services::user_service::UserService,
};
use anyhow::Result;

pub struct RegisterUserUserCase<T: UserRepository> {
    user_service: UserService<T>,
}

impl<T: UserRepository> RegisterUserUserCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        Self { user_service }
    }

    pub async fn execute(&self, new_user: &NewUserDTO) -> Result<()> {
        self.user_service.register_user(new_user).await
    }
}
