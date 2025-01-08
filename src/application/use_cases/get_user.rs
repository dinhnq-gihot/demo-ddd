use crate::domain::{
    entities::user::User,
    repositories::user_repository::UserRepository,
    services::user_service::UserService,
};

pub struct GetUserUserCase<T: UserRepository> {
    user_service: UserService<T>,
}

impl<T: UserRepository> GetUserUserCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        Self { user_service }
    }

    pub async fn get_by_email(&self, email: String) -> Option<User> {
        self.user_service.get_by_email(email).await
    }
}
