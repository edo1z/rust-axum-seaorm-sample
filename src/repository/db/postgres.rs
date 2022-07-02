pub mod post_postgres;
pub mod user_postgres;

use crate::domain::Repositories;
use dotenv::dotenv;
use post_postgres::PostRepo;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::sync::Arc;
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
    pub async fn new() -> Self {
        let conn = Arc::new(connect().await);
        Self {
            user: UserRepo::new(conn.clone()),
            post: PostRepo::new(conn),
        }
    }
}

pub async fn connect() -> DatabaseConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(db_url)
        .await
        .expect("Database connection failed")
}
