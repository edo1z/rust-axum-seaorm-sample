pub mod cors;

use crate::repository::db::postgres;
use crate::router::router;
use crate::usecase::create_usecases;
use axum::{extract::Extension, Router};
use cors::cors;
use std::sync::Arc;

pub async fn create_app() -> Router {
    let repo = Arc::new(postgres::create_repositories().await);
    let usecases = Arc::new(create_usecases(repo));
    router().layer(cors()).layer(Extension(usecases))
}
