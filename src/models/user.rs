use serde::{ Serialize, Deserialize };
use validator::Validate;

// main User model
#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub age: i32
}

impl User {
    pub fn new(uuid: String, username: String, password: String, age: i32) -> User {
        User {uuid, username, password, age}
    }
}

// models for validation
// may not be needed yet

#[derive(Serialize, Deserialize, Validate)]
pub struct UserCreds {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 10, max = 50))]
    pub password: String,
    #[validate(range(min = 18, max = 100))]
    pub age: i32
}