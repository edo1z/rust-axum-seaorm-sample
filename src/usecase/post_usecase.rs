use crate::domain::{model::post_model::Post, post_domain::PostUsecase};
use crate::repository::Repositories;
use async_trait::async_trait;
use std::sync::Arc;

pub struct PostUsecaseImpl {
    repo: Arc<Repositories>,
}
impl PostUsecaseImpl {
    pub fn new(repo: Arc<Repositories>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl PostUsecase for PostUsecaseImpl {
    async fn get_all(&self) -> Vec<Post> {
        self.repo.post_repo.get_all().await
    }
}
