use crate::domain::model::post_model::Post;
use async_trait::async_trait;

#[async_trait]
pub trait PostUsecase {
    async fn get_all(&self) -> Vec<Post>;
}

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn get_all(&self) -> Vec<Post>;
}
