use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
    pub id: String,
    pub title: String,
}
