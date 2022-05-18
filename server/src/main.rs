mod configs;

use crate::configs::Config;

use dotenv::dotenv;  


fn main() {
    dotenv().ok();

    let config = Config::from_env().unwrap();
    let pool = config.configure_pool();
    let log = Config::configure_log();
    let mongo_client = Config::configure_mongo_client(format!("mongodb://{}:{}", config.mongo.host, config.mongo.port));

    
}