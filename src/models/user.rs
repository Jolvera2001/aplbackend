use serde::{ Serialize, Deserialize };
use validator::Validate;

// main User model
#[derive(Serialize, Deserialize, Validate)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub age: i32
}

impl User {
    pub fn new (uuid: String, username: String, password: String, age: i32) -> User {
        User {uuid, username, password, age}
    }
}

// models for validation
// may not be needed yet