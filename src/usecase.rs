pub mod post_usecase;
pub mod user_usecase;

use crate::domain::post_domain::PostUsecase;
use crate::domain::user_domain::UserUsecase;

use crate::repository::Repositories;
use post_usecase::PostUsecaseImpl;
use std::sync::Arc;
use user_usecase::UserUsecaseImpl;

pub struct Usecases {
    pub user_usecase: Box<dyn UserUsecase>,
    pub post_usecase: Box<dyn PostUsecase>,
}

pub fn create_usecases(repo: Arc<Repositories>) -> Usecases {
    let user_usecase = Box::new(UserUsecaseImpl::new(repo.clone()));
    let post_usecase = Box::new(PostUsecaseImpl::new(repo));
    Usecases {
        user_usecase,
        post_usecase,
    }
}
