use crate::router::router;
use axum::Router;

pub async fn create_app() -> Router {
    router()
}
