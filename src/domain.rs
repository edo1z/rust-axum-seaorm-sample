pub mod post_domain;
pub mod user_domain;

use post_domain::PostRepository;
use user_domain::UserRepository;

pub trait Repositories: Send + Sync {
    type UserRepo: UserRepository;
    type PostRepo: PostRepository;
    fn user(&self) -> &Self::UserRepo;
    fn post(&self) -> &Self::PostRepo;
}
