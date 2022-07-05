pub mod post_usecase;
pub mod user_usecase;

use crate::repository::Repositories;
use post_usecase::PostUsecaseImpl;
use std::sync::Arc;
use user_usecase::UserUsecaseImpl;

pub struct Usecases {
    pub user_usecase: UserUsecaseImpl,
    pub post_usecase: PostUsecaseImpl,
}
impl Usecases {
    pub fn new(repo: Arc<Repositories>) -> Self {
        let user_usecase = UserUsecaseImpl::new(repo.clone());
        let post_usecase = PostUsecaseImpl::new(repo);
        Self {
            user_usecase,
            post_usecase,
        }
    }
}
