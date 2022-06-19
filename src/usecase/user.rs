use crate::domain::user::User;

pub async fn get_all() -> Vec<User> {
    let user = User {
        id: String::from("abc"),
        name: String::from("Taro"),
        age: Some(15),
    };
    vec![user]
}
