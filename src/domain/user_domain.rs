use crate::domain::model::user_model::Model as User;
use async_trait::async_trait;

#[async_trait]
pub trait UserUsecase {
    async fn get_all(&self) -> Vec<User>;
    async fn get_by_id(&self, id: i32) -> Result<User, String>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_all(&self) -> Vec<User>;
    async fn get_by_id(&self, id: i32) -> Result<User, String>;
}
