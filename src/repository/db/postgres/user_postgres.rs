use crate::domain::{model::user_model::User, user_domain::UserRepository};
use async_trait::async_trait;

pub struct UserRepo;
#[async_trait]
impl UserRepository for UserRepo {
    async fn get_all(&self) -> Vec<User> {
        let user = User {
            id: String::from("abc"),
            name: String::from("Taro"),
            age: Some(15),
        };
        vec![user]
    }
}
