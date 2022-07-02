use crate::domain::{model::post_model::Post, post_domain::PostRepository};
use async_trait::async_trait;

pub struct PostRepo;
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
