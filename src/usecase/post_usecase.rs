use crate::domain::post_domain::{Post, PostRepository, PostUsecase};
use crate::domain::Repositories;
use async_trait::async_trait;
use std::sync::Arc;

pub struct PostUsecaseImpl<R: Repositories> {
    repo: Arc<R>,
}
impl<R: Repositories> PostUsecaseImpl<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: Repositories> PostUsecase for PostUsecaseImpl<R> {
    async fn get_all(&self) -> Vec<Post> {
        self.repo.post().get_all().await
    }
}
