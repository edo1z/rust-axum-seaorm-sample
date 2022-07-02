pub mod post_usecase;
pub mod user_usecase;

use crate::domain::Repositories;
use post_usecase::PostUsecaseImpl;
use std::sync::Arc;
use user_usecase::UserUsecaseImpl;

pub struct UsecasesImpl<R: Repositories> {
    pub user: UserUsecaseImpl<R>,
    pub post: PostUsecaseImpl<R>,
}

pub trait Usecases {
    type Repo: Repositories;
    fn new(repo: Arc<Self::Repo>) -> UsecasesImpl<Self::Repo> {
        let user = UserUsecaseImpl::new(repo.clone());
        let post = PostUsecaseImpl::new(repo.clone());
        UsecasesImpl { user, post }
    }
    fn user(&self) -> &UserUsecaseImpl<Self::Repo>;
    fn post(&self) -> &PostUsecaseImpl<Self::Repo>;
}
