mod configs;
mod handlers;
mod models;

use crate::configs::Config;
use crate::handlers::*;
use crate::models::AppData;

use dotenv::dotenv;  
use slog::{info};
use actix_web::{HttpServer, App, web};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // configure logger, server, mongodb, postgres
    let config = Config::from_env().unwrap();
    let pool = config.configure_pool();
    let log = Config::configure_log();
    let mongo_client = Config::configure_mongo_client(format!("mongodb://{}:{}", config.mongo.host, config.mongo.port));

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(
                    AppData { // pass data in App
                        pool: pool.clone(),
                        log: log.clone(),
                        mongo_client: mongo_client.clone()
                    }
                )
            )
            .route("/", web::get().to(status))  
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
