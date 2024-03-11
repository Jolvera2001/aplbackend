use actix_web::{ get, post, patch, App, HttpResponse, HttpServer, Responder, web::Json, web::Path, web::Data };
use validator::Validate;
use uuid;

use crate::models::{User, UserCreds};


// main code
#[post("/user/register")]
pub async fn register_user(body: Json<UserCreds>) -> impl Responder {
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
            let new_user = User::new(
                String::from(new_uuid), 
                username, 
                password, 
                age);

            // Response
            HttpResponse::Ok().body(format!("User Registered: {} {} {}", new_user.username, new_user.password, new_user.age))
        },
        Err(_e) => {
            // Error response
             HttpResponse::BadRequest().body(format!("Invalid Request"))
        }
    }
}