use actix_web::{ get, post, patch, App, HttpResponse, HttpServer, Responder, web::Json, web::Path, web::Data };

#[get("/test")]
pub async fn test_api() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}