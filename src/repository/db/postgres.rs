pub mod post_postgres;
pub mod user_postgres;

use crate::repository::Repositories;
use dotenv::dotenv;
use post_postgres::PostRepo;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::sync::Arc;
use user_postgres::UserRepo;

pub async fn create_repositories() -> Repositories {
    let conn = Arc::new(connect().await);
    let user_repo = Box::new(UserRepo::new(conn.clone()));
    let post_repo = Box::new(PostRepo::new(conn));
    Repositories::new(user_repo, post_repo)
}

pub async fn connect() -> DatabaseConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(db_url)
        .await
        .expect("Database connection failed")
}
