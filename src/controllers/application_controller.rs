use actix_web::{ get, post, patch, delete, App, HttpResponse, HttpServer, Responder, web::Json, web::Path, web::Data };
use validator::Validate;
use uuid;

// importing models
use crate::models::Application;

#[get("/application")]
pub async fn get_applications() -> impl Responder {
    Todo!("Implement to retrieve ALL applications of a user")
}

#[post("/application/add")]
pub async fn add_application() -> impl Responder {
    Todo!("Needs application model")
}

#[patch("/application/edit")]
pub async fn edit_application() -> Responder {
    Todo!("Needs application model; needs to be able to edit application")
}

#[delete("/application/delete")]
pub async fn delete_application() -> Responder {
    Todo!("Delete application using application id")
}