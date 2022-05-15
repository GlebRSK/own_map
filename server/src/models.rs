use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;


#[derive(Serialize)]
pub struct Status {
    pub status: String
}


#[derive(Deserialize)]
struct GeoPoint {
    lat: f64,
    long: f64,
    timestamp: String,
    activity: String, 
}

impl GeoPoint {

    fn new(point: &serde_json::Value) -> Self {
        
        let latitude: f64 = match point.get("latitudeE7").unwrap().as_f64() {
            Some(l) => l/10000000.0,
            None    => 0.0,
        };
        
        let longitude: f64 = match point.get("longitudeE7").unwrap().as_f64() {
            Some(l) => l/10000000.0,
            None    => 0.0,
        };

        let tstamp: String = match point.get("timestamp").unwrap().as_str() {
            Some(tstamp) => tstamp.to_string(),
            None         => "".to_string()
        };
        
        let activity: String = GeoPoint::extract_activity(&point);

        GeoPoint {
            lat: latitude,
            long: longitude,
            timestamp: tstamp,
            activity: activity,
        }
    }

    fn extract_activity(point: &serde_json::Value) -> String{
    
        let activity = match point.get("activity") {
            Some(activities) => {        
                let mut type_of_action = "".to_string();
                for action in activities.as_array() {
                    for act in action{
                        type_of_action = match act.get("activity").unwrap()[0].get("type").unwrap().as_str() {
                            Some(t) => t.to_string(),
                            None => "Missing".to_string()
                        };
                    }
                }
    
                type_of_action
            },
            None      => "missing".to_string()
        };
        
        activity
    }
    
    fn copy(gp: &GeoPoint) -> GeoPoint{

        GeoPoint {
            lat: gp.lat,
            long: gp.long,
            timestamp: (*gp.timestamp).to_string(),
            activity: (*gp.activity).to_string()
        }

    }
}