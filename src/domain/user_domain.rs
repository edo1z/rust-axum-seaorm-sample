use crate::domain::model::user_model::Model as User;
use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserUsecase: Send + Sync {
    async fn get_all(&self) -> Vec<User>;
    async fn get_by_id(&self, id: i32) -> Result<Option<User>>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_all(&self) -> Vec<User>;
    async fn get_by_id(&self, id: i32) -> Result<Option<User>>;
}
