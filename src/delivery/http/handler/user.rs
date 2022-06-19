use crate::domain::user::User;
use crate::usecase::user::get_all;
use axum::Json;

pub async fn index() -> Json<Vec<User>> {
    let users = get_all().await;
    Json(users)
}
