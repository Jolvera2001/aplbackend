#![allow(unused_imports)]
use actix_web::{ get, App, HttpResponse, HttpServer, Responder, web::Data };

// local crate imports
mod controllers;
mod models;
mod db;

// imported structs
use controllers::{ register_user, login_user };
use db::Database;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("Failed to connect to database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(index)
            .service(register_user)
            .service(login_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
