use crate::domain::{model::user_model::User, user_domain::UserRepository};
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct UserRepo {
    pub conn: Arc<DatabaseConnection>,
}
impl UserRepo {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}
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
