use surrealdb::engine::remote::ws::{ Client, Ws };
use surrealdb::opt::auth::Root;
use surrealdb::{ Error, Surreal };

use crate::models::user::User;
use crate::models::application::Application;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client.signin(Root {
            username: "root",
            password: "root"
        })
        .await?;
        client.use_ns("surreal").use_db("aplcore").await.unwrap();
        Ok(Database {
            client, name_space: String::from("surreal"),
            db_name: String::from("aplcore")
        })
    }

    pub async fn add_user(&self, new_user: User) -> Option<User> {
        let check_user: Result<Option<User>, Error> = self.client
            .select(("users", new_user.username.clone()))
            .await;

        match check_user {
            Ok(Some(_)) => return None,
            _ => {
                let created_user = self.client
                    .create(("users", new_user.username.clone()))
                    .content(new_user)
                    .await;

                match created_user {
                    Ok(user) => user,
                    Err(_) => None,
                }
            }
        }
        
    }

    pub async fn check_user(&self, username: String, password: String) -> Option<User> {
        let user_result: Result<Option<User>, Error> = self.client
            .select(("users", username))
            .await;

        match user_result {
            Ok(Some(user)) => {
                if user.password == password {
                    Some(user)
                } else {
                    None
                }
            },
            _ => None
        }
    }

    pub async fn get_user_applications(&self, username: String) -> Result<Option<Vec<Application>>, Error> {
        todo!("Get application records using IDs within application_ids")
    }

    pub async fn add_user_applications(&self, username: String, application_add: Application) -> Result<Option<User>, Error> {
        // todo!("Add application record and update user application_ids field with new application ID")
        let check_user: Result<Option<User>, Error> = self.client
            .select(("users", username.clone()))
            .await;

        match check_user {
            Ok(Some(mut user)) => {
                let created_application: Result<Option<Application>, Error> = self.client
                    .create(("applications", application_add.uuid.clone()))
                    .content(application_add)
                    .await;

                match created_application {
                    Ok(application) => {
                        if let Some(application_ids) = &mut user.application_ids {
                            application_ids.push(application.unwrap().uuid.clone());
                        } else {
                            user.application_ids = Some(vec![application.unwrap().uuid.clone()]);
                        }
                        let updated_user = self.client
                            .update(("users", username.clone()))
                            .content(user)
                            .await;

                        match updated_user {
                            Ok(Some(user)) => Ok(Some(user)),
                            Ok(None) => Ok(None),
                            Err(e) => Err(e),
                        }
                    },
                    Err(e) => Err(e),
                }
            },
            Ok(None) => Ok(None),
            Err(e) => Err(e)
        }
    }
}