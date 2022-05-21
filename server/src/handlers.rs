use crate::models::{Status, AppData, CreateGeoLocation};
use crate::error::{AppError};
use crate::db;

use deadpool_postgres::{Pool, Client};
use slog::{Logger, o, info, crit, error};  
use actix_web::{Responder, HttpResponse, web};


pub async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
   
    pool.get().
        await.
        map_err( |err| {
            let sublog = log.new(o!("cause" => err.to_string()));
            crit!(sublog, "Error getting client!");
            AppError::db_error(err)
        })
}

pub fn log_error(log: Logger) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        let sublog = log.new(o!("cause" => err.cause.clone()));
        error!(sublog, "{}", err.message());
        err
    })
}

pub async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(Status {status: "Ok".to_string()})
}

pub async fn get_locations(state: web::Data<AppData>) -> Result<impl Responder, AppError> {

    let log = state.log.new( o!("handler" => "get_locations"));
    
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::get_locations(&client).await;

    result.map(|points| HttpResponse::Ok().json(points))
        .map_err(log_error(log))
}

pub async fn create_locations(state: web::Data<AppData>, json: web::Json<CreateGeoLocation>) ->  Result<impl Responder, AppError> {

    let log = state.log.new( o!("handler" => "create_locations"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    info!(log, "{}, {}, {}, {}",  json.lat, json.long, json.timestamp.clone(), json.activity.clone());
    let result = db::insert_point(&client, json.lat, json.long, json.timestamp.clone(), json.activity.clone()).await;

    result.map(|point| HttpResponse::Ok().json(point))
        .map_err(log_error(log))
}