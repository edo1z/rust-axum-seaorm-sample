use crate::delivery::http::handler::{root_handler, user_handler};
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/", get(root_handler::index))
        .nest("/users", user_routes())
}

fn user_routes() -> Router {
    Router::new().route("/", get(user_handler::index))
}
