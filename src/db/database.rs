use surrealdb::engine::remote::ws::{ Client, Ws };
use surrealdb::opt::auth::Root;
use surrealdb::{ Error, Surreal };
use crate::models::user::User;

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
        let created_user = self.client
            .create(("users", new_user.username.clone()))
            .content(new_user)
            .await;

        match created_user {
            Ok(user) => user,
            Err(_) => None,
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
}