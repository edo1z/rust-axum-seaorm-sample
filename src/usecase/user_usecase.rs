use crate::domain::Repositories;
use crate::domain::{
    model::user_model::Model as User,
    user_domain::{UserRepository, UserUsecase},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct UserUsecaseImpl<R: Repositories> {
    repo: Arc<R>,
}
impl<R: Repositories> UserUsecaseImpl<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: Repositories> UserUsecase for UserUsecaseImpl<R> {
    async fn get_all(&self) -> Vec<User> {
        self.repo.user().get_all().await
    }
    async fn get_by_id(&self, id: i32) -> Result<Option<User>> {
        let users = self.repo.user().get_by_id(id).await?;
        Ok(users)
    }
}
