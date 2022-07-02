pub mod post_postgres;
pub mod user_postgres;

use crate::domain::Repositories;
use post_postgres::PostRepo;
use user_postgres::UserRepo;

pub struct Repo {
    pub user: UserRepo,
    pub post: PostRepo,
}
impl Repositories for Repo {
    type UserRepo = UserRepo;
    type PostRepo = PostRepo;
    fn user(&self) -> &Self::UserRepo {
        &self.user
    }
    fn post(&self) -> &Self::PostRepo {
        &self.post
    }
}
impl Repo {
    pub fn new() -> Self {
        Self {
            user: UserRepo {},
            post: PostRepo {},
        }
    }
}
