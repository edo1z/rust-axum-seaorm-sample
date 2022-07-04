use super::super::viewmodel::user_viewmodel::UserId;
use super::response::{AppError, Result};
use crate::bootstrap::ExtUsecases;
use crate::domain::{model::user_model::Model as User, user_domain::UserUsecase};
use crate::usecase::Usecases;
use axum::{
    extract::{Extension, Path},
    Json,
};

pub async fn index(Extension(usecases): ExtUsecases) -> Result<Json<Vec<User>>> {
    let user_usecase = usecases.user();
    let users = user_usecase.get_all().await;
    Ok(Json(users))
}

pub async fn get_by_id(
    Extension(usecases): ExtUsecases,
    Path(id): Path<UserId>,
) -> Result<Json<User>> {
    match usecases.user().get_by_id(id).await? {
        None => Err(AppError::NotFound("User is not found.")),
        Some(user) => Ok(Json(user)),
    }
}
