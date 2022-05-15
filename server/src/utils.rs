use crate::models::{GeoPoint}; 

fn parse_json() -> Vec<GeoPoint> {
    
    let filename = "Records.json";
    let data = fs::read_to_string(filename).expect("Unable to file"); 
    let stream = Deserializer::from_str(&data).into_iter::<Value>();
    let mut points: Vec<GeoPoint> = Vec::new();
    
    for value in stream {
        let sequence = value.unwrap()["locations"].clone();
        for data in sequence.as_array() {
            for point in data {                        
                points.push(GeoPoint::new(point));
            }              
        }
    }

    points
}