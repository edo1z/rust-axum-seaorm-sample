use crate::domain::model::user_model::Model as User;

pub fn user_fixture() -> User {
    User {
        id: 1,
        name: String::from("Taro"),
        age: Some(20),
    }
}
