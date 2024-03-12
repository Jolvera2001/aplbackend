use actix_web::{ get, post, patch, HttpResponse, Responder, web::Json, web::Path, web::Data };
use validator::Validate;
use uuid;

use crate::models::{User, UserCreds, UserLogin };
use crate::db::Database;

#[post("/user/register")]
pub async fn register_user(body: Json<UserCreds>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            // we create the user model
            let username = body.username.clone();
            let password = body.password.clone();
            let age = body.age;

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            // when using a database, use db.func(user::new())
            let new_user = db.add_user(User::new(
                String::from(new_uuid),
                username,
                password,
                age
            )).await;

            match new_user {
                Some(user) => return HttpResponse::Ok().body(format!("Created new user: {:?}", user)),
                None => return HttpResponse::BadRequest().body(format!("Failed to create new user"))
            }
        },
        Err(_e) => {
            // Error response
             HttpResponse::BadRequest().body(format!("Invalid Request"))
        }
    }
}

#[post("/user/login")]
pub async fn login_user(body: Json<UserLogin>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let username = body.username.clone();
            let password = body.password.clone();

            let login_user = db.check_user(username, password).await;

            match login_user {
                Some(user) => return HttpResponse::Ok().body(format!("Logged in as: {:?}", user)),
                None => return HttpResponse::BadRequest().body(format!("Failed to login"))
            }
        }
        Err(_e) => {
            HttpResponse::BadRequest().body(format!("Invalid Request"))
        }
    }
}