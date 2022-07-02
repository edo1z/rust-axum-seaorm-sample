use axum::http::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer};

pub fn cors() -> CorsLayer {
    let origins = ["http://localhost:8001".parse().unwrap()];
    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods(Any)
        .allow_headers(vec![CONTENT_TYPE])
}
