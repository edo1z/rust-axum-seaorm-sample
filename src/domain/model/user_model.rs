// use sea_orm::entity::prelude::*;
use serde::Serialize;

//DeriveEntityModel
#[derive(Serialize, Clone, Debug, PartialEq)]
// #[sea_orm(table_name = "users")]
pub struct User {
    // #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub age: Option<u8>,
}

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}
// impl ActiveModelBehavior for ActiveModel {}
