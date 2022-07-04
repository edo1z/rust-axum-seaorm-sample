use crate::domain::{
    model::user_model::{Entity as UserEntity, Model as User},
    user_domain::UserRepository,
};
use anyhow::Result;
use async_trait::async_trait;
use sea_orm::{prelude::*, DatabaseConnection};
use std::sync::Arc;

pub struct UserRepo {
    pub conn: Arc<DatabaseConnection>,
}
impl UserRepo {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}
#[async_trait]
impl UserRepository for UserRepo {
    async fn get_all(&self) -> Vec<User> {
        UserEntity::find().all(&*self.conn).await.unwrap()
    }
    async fn get_by_id(&self, id: i32) -> Result<Option<User>> {
        let user = UserEntity::find_by_id(id).one(&*self.conn).await?;
        Ok(user)
    }
}
