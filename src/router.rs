use crate::delivery::http::handler::{root, user};
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/", get(root::index))
        .nest("/users", user_routes())
}

fn user_routes() -> Router {
    Router::new().route("/", get(user::index))
}
