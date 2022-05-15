use crate::geo_point::{GeoPoint};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;


pub fn async injection_start_data(client: &Client, geo_point: Vec<GeoPoint>) -> Result<(), io::Error> {


    let statement = client
                        .prepare("insert into geo_point (lat, long, timestamp, activity) values ($1, $2, $3, $4) returning id, lat, long, timestamp, activity")
                        .await.unwrap();

    for point in geo_point {

        client.query(&statement, &[&point.lat. &point.long, &point.timestamp, point.activity])
            .await
            .expect("Error create new point")
            .iter()
            .map(|row| GeoPoint::from_row_ref(row).unwrap())
            .collect::<Vec<GeoPoint>>()
            .pop()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating new point"));
    }

    Ok(())
}