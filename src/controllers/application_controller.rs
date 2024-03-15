use actix_web::{ get, post, patch, delete, App, HttpResponse, HttpServer, Responder, web::Json, web::Path, web::Data };
use validator::Validate;
use uuid;

use crate::models::{ Application, UpdateApplicationURL, ApplicationCreds };
use crate::db::Database;

#[get("/application/{user}")]
pub async fn get_applications(user_record: Path<String>, db: Data<Database>) -> impl Responder {
      let applications = db.get_user_applications(user_record.into_inner()).await;
      match applications {
          Ok(Some(apps)) => HttpResponse::Ok().json(apps),
          Ok(None) => HttpResponse::Ok().body("No applications found"),
          Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve applications"),
      }
}

#[post("/application/add/{user}")]
pub async fn add_application(user_record: Path<String>, body: Json<ApplicationCreds>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let user = user_record.clone();
            let job_title = body.job_title.clone();
            let job_description = body.job_description.clone();
            let job_status = body.job_status.clone();
            let date_created = body.date_created.clone();
            let job_closed = body.job_closed;
            let job_source = body.job_source.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_application = db.add_user_applications(user.clone(), Application::new(
                String::from(new_uuid),
                job_title,
                job_description,
                job_status,
                date_created,
                job_closed,
                job_source,
                user
            )).await;

            match new_application {
                Ok(user) => HttpResponse::Ok().body(format!("Added new application to user: {:?}", user)),
                Err(_) => HttpResponse::BadRequest().body(format!("Failed to create new application"))
            }
        }
        Err(_e) => {
            HttpResponse::BadRequest().body(format!("Invalid Request"))
        }
    }
}

// #[patch("/application/edit/{uuid}")]
// pub async fn edit_application(application_url: Path<UpdateApplicationURL>) -> Responder {
//     let uuid = application_url.into_inner().uuid;
//     HttpResponse::Ok().body(format!("Updating application with uuid: {}", uuid))
// }

// #[delete("/application/delete")]
// pub async fn delete_application(body: Json<Application>) -> Responder {
//     todo!("Delete application using application id")
// }