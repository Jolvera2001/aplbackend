use actix_web::{ get, App, HttpResponse, HttpServer, Responder, web::Data };

// local crate imports
mod controllers;
mod models;

// imported controllers
use controllers::{ test_api, register_user };

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(register_user)
            .service(test_api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
