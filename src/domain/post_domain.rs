use async_trait::async_trait;
use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
    pub id: String,
    pub title: String,
}

#[async_trait]
pub trait PostUsecase {
    async fn get_all(&self) -> Vec<Post>;
}

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn get_all(&self) -> Vec<Post>;
}
