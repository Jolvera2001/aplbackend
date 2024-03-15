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
        let check_user: Result<Option<User>, Error> = self.client
            .select(("users", username.clone()))
            .await;

        match check_user {
            Ok(_) => {
                let sql = format!("SELECT * FROM applications WHERE user = {}", username);
                let user_applications = self.client
                    .query(sql)
                    .await;

                match user_applications {
                    Ok(mut response) => {
                        let application_vector: Option<Vec<Application>> = response.take(0)?;
                        Ok(application_vector) 
                    }
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }

    }

    pub async fn add_user_applications(&self, username: String, new_application: Application) -> Result<Option<Application>, Error> {
        let check_user: Result<Option<User>, Error> = self.client
            .select(("users", username.clone()))
            .await;

        match check_user {
            // if there is a valid user, then we run the process to add
            Ok(_) => {
                let add_application = self.client
                    .create(("applications", new_application.uuid.clone()))
                    .content(new_application)
                    .await; 

                match add_application {
                    Ok(application) => Ok(application),
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e)
        }

    }
}