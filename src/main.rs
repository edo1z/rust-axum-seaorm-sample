mod bootstrap;
mod delivery;
mod domain;
mod repository;
mod router;
#[cfg(test)]
mod test;
mod usecase;

#[tokio::main]
async fn main() {
    let app = bootstrap::create_app().await;
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
