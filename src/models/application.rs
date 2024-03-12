use serde::{ Serialize, Deserialize };
use validator::Validate;

// main User model
#[derive(Serialize, Deserialize, Validate)]
pub struct Application {
    pub uuid: String,
    pub job_title: String,
    pub job_description: String,
    pub job_status: String,
    pub date_created: String,
    pub job_closed: bool,
    pub job_source: String
}

impl Application {
    pub fn new(uuid: String, job_title: String, job_description: String, job_status: String, date_created: String, job_closed: bool, job_source: String) -> Application {
        Application {uuid, job_title, job_description, job_status, date_created, job_closed, job_source}
    }
}

// models for validation
// may not be needed yet