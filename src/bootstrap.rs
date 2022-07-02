use crate::repository::db::postgres::Repo;
use crate::router::router;
use crate::usecase::Usecases;
use crate::usecase::UsecasesImpl;
use axum::{extract::Extension, Router};
use std::sync::Arc;
// use crate::domain::Repositories;
use crate::usecase::post_usecase::PostUsecaseImpl;
use crate::usecase::user_usecase::UserUsecaseImpl;

pub async fn create_app() -> Router {
    let repo = Arc::new(Repo::new());
    let usecases = Arc::new(UsecasesImpl::<Repo>::new(repo));
    let ext_usecase: ExtUsecases = Extension(usecases);
    router().layer(ext_usecase)
}

pub type ExtUsecases = Extension<Arc<UsecasesImpl<Repo>>>;

impl Usecases for UsecasesImpl<Repo> {
    type Repo = Repo;
    fn user(&self) -> &UserUsecaseImpl<Repo> {
        &self.user
    }
    fn post(&self) -> &PostUsecaseImpl<Repo> {
        &self.post
    }
}
