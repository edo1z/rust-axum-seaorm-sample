use async_trait::async_trait;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: Option<i16>,
}

#[async_trait]
pub trait UserUsecase {
    async fn get_all(&self) -> Vec<User>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_all(&self) -> Vec<User>;
}
