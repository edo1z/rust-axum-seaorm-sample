pub mod db;

use crate::domain::post_domain::PostRepository;
use crate::domain::user_domain::UserRepository;

pub struct Repositories {
    pub user_repo: Box<dyn UserRepository>,
    pub post_repo: Box<dyn PostRepository>,
}
impl Repositories {
    pub fn new(user_repo: Box<dyn UserRepository>, post_repo: Box<dyn PostRepository>) -> Self {
        Self {
            user_repo,
            post_repo,
        }
    }
}
