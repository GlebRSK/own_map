use serde::{Serialize, Deserialize};
use deadpool_postgres::Pool;
use slog::Logger;
//use mongodb::{options::ClientOptions};

/*
The data structure that is passes on actix_web App
*/
#[derive(Clone)]
pub struct AppData {
    pub pool: Pool,
    pub log: Logger,
   // mongo_client: ClientOptions,
}


/*
Get status of server connection
*/
#[derive(Serialize)]
pub struct Status {
    pub status: String,
}