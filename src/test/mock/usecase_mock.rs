use crate::domain::{post_domain::MockPostUsecase, user_domain::MockUserUsecase};
use crate::usecase::Usecases;

pub fn create_usecases_for_mock() -> Usecases {
    Usecases {
        user_usecase: Box::new(MockUserUsecase::new()),
        post_usecase: Box::new(MockPostUsecase::new()),
    }
}
