use crate::domain::model::post_model::Post;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait PostUsecase: Send + Sync {
    async fn get_all(&self) -> Vec<Post>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn get_all(&self) -> Vec<Post>;
}
