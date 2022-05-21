use crate::models::{GeoLocation};
use crate::error::{AppError, AppErrorType};

use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;


pub async fn get_locations(client: &Client) -> Result<Vec<GeoLocation>, AppError> {
    let statement = client
        .prepare("select * from geo_locations")
        .await
        .map_err(AppError::db_error)?;

    let geo_point = client.query(&statement, &[])
        .await
        .expect("Error getting geo point")
        .iter()
        .map(|point| GeoLocation::from_row_ref(point).unwrap())
        .collect::<Vec<GeoLocation>>();

    Ok(geo_point)
}

pub async fn insert_point(client: &Client, lat: f32, long: f32, tstmp: String, activity: String) -> Result<GeoLocation, AppError> {
    let statment = client
        .prepare("insert into geo_locations (lat, long, timestamp, activity) values ($1,$2,$3,$4) returning id, lat, long, timestamp, activity")
        .await
        .map_err(AppError::db_error)?;
        
        client.query(&statment, &[&lat, &long, &tstmp, &activity])
            .await
            .expect("Error create point")
            .iter()
            .map(|point| GeoLocation::from_row_ref(point).unwrap())
            .collect::<Vec<GeoLocation>>()
            .pop()
            .ok_or(AppError{
                message: Some("Error creating Geo Point".to_string()),
                cause: Some("Unknown error".to_string()),
                error_type: AppErrorType::DBError
            })
}