use actix_web::{ get, post, patch, delete, App, HttpResponse, HttpServer, Responder, web::Json, web::Path, web::Data };
use validator::Validate;
use uuid;

use crate::models::{ Application, UpdateApplicationURL };

// #[get("/application")]
// pub async fn get_applications() -> impl Responder {
//     todo!("Implement to retrieve ALL applications of a user")
// }

// #[post("/application/add")]
// pub async fn add_application() -> impl Responder {
//     todo!("Needs application model")
// }

// #[patch("/application/edit/{uuid}")]
// pub async fn edit_application(application_url: Path<UpdateApplicationURL>) -> Responder {
//     let uuid = application_url.into_inner().uuid;
//     HttpResponse::Ok().body(format!("Updating application with uuid: {}", uuid))
// }

// #[delete("/application/delete")]
// pub async fn delete_application(body: Json<Application>) -> Responder {
//     todo!("Delete application using application id")
// }