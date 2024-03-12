use serde::{ Serialize, Deserialize };
use validator::Validate;
use surrealdb::Error;

use crate::db::Database;
use crate::models::traits::CrudOperations;

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

// CRUD

impl CrudOperations<User> for Database {
    async fn create(&self, new_item: User) -> Result<(), Error> {
        let created_user = self.client
            .create(("users", new_item.uuid.clone()))
            .content(new_item)
            .await;

        match created_user {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn read(&self, id: String) -> Result<Option<User>, Error> {
        
    }

    async fn update(&self, id: String, item: User) -> Result<(), Error> {
        
    }

    async fn delete(&self, id: String) -> Result<(), Error> {
        
    }
}