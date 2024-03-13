use serde::{ Serialize, Deserialize };
use validator::Validate;
use surrealdb::Error;

use crate::db::Database;
use crate::models::application::Application;

// main User model
#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub age: i32,
    pub applications: Option<Vec<Application>> 
}

impl User {
    pub fn new(uuid: String, username: String, password: String, age: i32) -> User {
        User {uuid, username, password, age, applications: Some(Vec::new())}
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

#[derive(Serialize, Deserialize, Validate)]
pub struct UserLogin {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 10, max = 50))]
    pub password: String
}