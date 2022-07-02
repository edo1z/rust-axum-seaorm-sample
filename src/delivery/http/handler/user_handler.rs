use crate::bootstrap::ExtUsecases;
use crate::domain::{model::user_model::User, user_domain::UserUsecase};
use crate::usecase::Usecases;
use axum::{extract::Extension, Json};
use axum_macros::debug_handler;

#[debug_handler]
pub async fn index(Extension(usecases): ExtUsecases) -> Json<Vec<User>> {
    let user_usecase = usecases.user();
    let users = user_usecase.get_all().await;
    Json(users)
}
