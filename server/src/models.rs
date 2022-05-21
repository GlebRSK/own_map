use serde::{Serialize, Deserialize};
use deadpool_postgres::Pool;
use slog::Logger;
use mongodb::{sync::Client, error::Error as MongoError};
use tokio_pg_mapper_derive::PostgresMapper;
/*
The data structure that is passes on actix_web App
*/
#[derive(Clone)]
pub struct AppData {
    pub pool: Pool,
    pub log: Logger,
    pub mongo_client: Result<Client, MongoError>,
}

/*
Get status of server connection
*/
#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

/*
Struct of geolocation point fron json
*/
#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="geo_locations")]
pub struct GeoLocation {
    pub id: i32,
    pub lat: f32,
    pub long: f32,
    pub timestamp: String,
    pub activity: String
}


/*
City struct
*/
pub struct City {
    pub id: i32,
    pub name: String,
    pub country: String
}

#[derive(Deserialize)]
pub struct CreateGeoLocation {
    pub lat: f32,
    pub long: f32,
    pub timestamp: String,
    pub activity: String
}