use crate::domain::user::User;
use axum::Json;

pub async fn index() -> Json<Vec<User>> {
    let user = User {
        id: String::from("abc"),
        name: String::from("Taro"),
        age: Some(15),
    };
    Json(vec![user])
}
