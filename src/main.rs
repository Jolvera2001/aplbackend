use actix_web::{ get, post, patch, App, HttpResponse, HttpServer, Responder, web::Json, web::Path, web::Data };
use validator::Validate;
use uuid;

// local crate imports
mod models;

// local structs
use crate::models::user::User;
use crate::models::user::UserCreds;
use crate::models::application::Application;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/user/register")]
async fn register_user(body: Json<UserCreds>) -> impl Responder {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(register_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
