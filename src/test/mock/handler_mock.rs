use crate::router;
use crate::usecase::Usecases;
use axum::{body::Body, extract::Extension, http::Request, response::Response};
use std::sync::Arc;
use tower::util::ServiceExt;

pub async fn request_mock(url: &'static str, body: Body, usecases: Arc<Usecases>) -> Response {
    let ext = Extension(usecases);
    let app = router::router().layer(ext);
    app.oneshot(Request::builder().uri(url).body(body).unwrap())
        .await
        .unwrap()
}
