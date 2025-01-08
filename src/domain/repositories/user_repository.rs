use crate::domain::entities::user::User;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: String) -> Option<User>;

    async fn save(&self, user: &NewUserDTO) -> Result<()>;
}
