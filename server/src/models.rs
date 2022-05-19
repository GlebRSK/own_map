use serde::{Serialize, Deserialize};
use deadpool_postgres::Pool;
use slog::Logger;
use mongodb::{sync::Client, error::Error as MongoError};

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