use crate::models::{Status};

use actix_web::{Responder, HttpResponse};

pub async fn status() -> impl Responder {

    HttpResponse::Ok()
        .json(Status {status: "Ok".to_string()})
}