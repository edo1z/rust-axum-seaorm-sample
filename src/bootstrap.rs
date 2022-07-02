pub mod cors;

use crate::repository::db::postgres::{connect, Repo};
use crate::router::router;
use crate::usecase::Usecases;
use crate::usecase::UsecasesImpl;
use axum::{extract::Extension, Router};
use cors::cors;
use std::sync::Arc;

pub async fn create_app() -> Router {
    let conn = connect().await;
    let repo = Arc::new(Repo::new(conn));
    let usecases = Arc::new(UsecasesImpl::<Repo>::new(repo));
    router().layer(cors()).layer(Extension(usecases))
}

pub type ExtUsecases = Extension<Arc<UsecasesImpl<Repo>>>;
