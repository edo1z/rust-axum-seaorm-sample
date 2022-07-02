use crate::domain::Repositories;
use crate::domain::{
    model::user_model::User,
    user_domain::{UserRepository, UserUsecase},
};
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
}
