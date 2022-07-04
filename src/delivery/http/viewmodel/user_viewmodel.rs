// use serde::Deserialize;
// use validator::Validate;

// #[derive(Debug, Validate, Deserialize)]
// pub struct UserId {
//     #[validate(range(min = 1, max = 100000000))]
//     pub id: i32,
// }
pub type UserId = i32;
