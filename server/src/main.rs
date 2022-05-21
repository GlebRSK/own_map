mod configs;
mod handlers;
mod models;
mod error;
mod db;

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

    info!(log, "Starting web server at http://{}:{}/", config.server.host, config.server.port);

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
            .route("/geopoint{_:/?}", web::get().to(get_locations))
            .route("/geopoint{_:/?}", web::post().to(create_locations))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

#[cfg(test)]
mod integration_tests {

    use crate::models::{AppData, GeoLocation};
    use crate::configs::Config;
    use crate::handlers::*;

    use actix_web::{App, web, test, http};
    use dotenv::dotenv;
    use lazy_static::lazy_static;
    use serde_json::json;

    lazy_static! {
        static ref APP_DATA: AppData = {
            dotenv().ok();

            let config = Config::from_env().unwrap();
            let pool = config.configure_pool();
            let log = Config::configure_log();
            let mongo_client = Config::configure_mongo_client(format!("mongodb://{}:{}", config.mongo.host, config.mongo.port));

            AppData {
                pool: pool.clone(),
                log: log.clone(),
                mongo_client: mongo_client.clone()
            }
        };
    }

    #[actix_rt::test]
    async fn get_geolocations() {
        let app = App::new()
            .app_data(web::Data::new(APP_DATA.clone()))
            .route("/geopoint{_:/?}", web::get().to(get_locations));
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::get()
            .uri("/geopoint")
            .to_request();
        let res = test::call_service(&mut app, req).await;
    
        assert_eq!(res.status(), 200, "GET /geopoint shold return status 200")
    }

    #[actix_rt::test]
    async fn test_create_points() {
        let app = App::new()
            .app_data(web::Data::new(APP_DATA.clone()))
            .route("/geopoint{_:/?}", web::get().to(get_locations))
            .route("/geopoint{_:/?}", web::post().to(create_locations));

        let mut app = test::init_service(app).await;
        
        //test create geopoint
        let lat = 1.0;
        let long = 2.0;
        let timestamp = "1970-01-01T00:00:00";
        let activity = "STILL";
        let create_geo_point = json!({
            "lat"       : lat,
            "long"      : long,
            "timestamp" : timestamp.clone(),
            "activity"  : activity.clone(),
        });

        let req = test::TestRequest::post()
            .uri("/geopoint")
            .insert_header(http::header::ContentType::json())
            .set_payload(create_geo_point.to_string())
            .to_request();

        let res = test::call_service(&mut app, req).await;
        assert_eq!(res.status(), 200, "POST /geopoint shold return status 200");

        let body = test::read_body(res).await;
        let try_created: Result<GeoLocation, serde_json::error::Error> = serde_json::from_slice(&body);
        assert!(try_created.is_ok(), "Response couldn't be parsed");

        let created_list = try_created.unwrap();

        // Test get created geopoint
        let req = test::TestRequest::get()
            .uri("/geopoint")  
            .to_request();

        let points: Vec<GeoLocation> = test::call_and_read_body_json(&mut app, req).await;
        let maybe_point = points
            .iter()
            .find(|point| point.id == created_list.id);
        assert!(maybe_point.is_some(), "Geolocation point is not found");
    }
}