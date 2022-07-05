use crate::domain::{model::user_model::Model as User, user_domain::UserUsecase};
use crate::repository::Repositories;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct UserUsecaseImpl {
    repo: Arc<Repositories>,
}
impl UserUsecaseImpl {
    pub fn new(repo: Arc<Repositories>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UserUsecase for UserUsecaseImpl {
    async fn get_all(&self) -> Vec<User> {
        self.repo.user_repo.get_all().await
    }
    async fn get_by_id(&self, id: i32) -> Result<Option<User>> {
        let users = self.repo.user_repo.get_by_id(id).await?;
        Ok(users)
    }
}
