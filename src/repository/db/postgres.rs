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
    pub fn new(conn: DatabaseConnection) -> Self {
        let arc_conn = Arc::new(conn);
        Self {
            user: UserRepo::new(arc_conn.clone()),
            post: PostRepo::new(arc_conn),
        }
    }
}

pub async fn connect() -> DatabaseConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    conn
}
