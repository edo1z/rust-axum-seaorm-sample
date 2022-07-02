use crate::domain::{model::post_model::Post, post_domain::PostRepository};
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct PostRepo {
    pub conn: Arc<DatabaseConnection>,
}
impl PostRepo {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}
#[async_trait]
impl PostRepository for PostRepo {
    async fn get_all(&self) -> Vec<Post> {
        let post = Post {
            id: String::from("abc"),
            title: String::from("Hello World!"),
        };
        vec![post]
    }
}
