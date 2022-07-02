use crate::domain::post_domain::{Post, PostRepository};
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
