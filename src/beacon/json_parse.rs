use axum::{extract, prelude::*, response};
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct BeaconData {
    date: String,
    os: OSType,
    r#type: String,
    beacons: Vec<Beacon>,
}

#[derive(Deserialize)]
pub struct Beacon {
    uuid: String,
    major: i64,
    minor: i64,
    rssi: f64,
    distance: f64,
}

#[derive(Deserialize)]
pub enum OSType {
    a,
    i,
    w,
    m,
}

impl OSType {
    fn check(&self) -> String {
        let result = match self {
            OSType::a => "a",
            OSType::i => "i",
            OSType::w => "w",
            OSType::m => "m",
        };
        result.to_string()
    }
}

#[derive(Serialize)]
pub struct BeaconResult {
    date: String,
    os: String,
    r#type: String,
    beacons: Vec<String>,
}

pub async fn get_beacon_data(
    extract::Json(get_beacon_data): extract::Json<BeaconData>,
) -> response::Json<BeaconResult> {

    let beacon_data = get_beacon_data;
    if beacon_data.beacons.len() == 0 {
        return response::Json(BeaconResult {
            date: beacon_data.date,
            os: beacon_data.os.check(),
            beacons: vec!["".to_string()],
            r#type: beacon_data.r#type,
        });
    }

    let beacon_id = beacon_data
        .beacons
        .iter()
        .map(|item| {
            (&item.uuid).to_string() + ":" + &item.major.to_string() + ":" + &item.minor.to_string()
        })
        .collect::<Vec<String>>();

    response::Json(BeaconResult {
        date: beacon_data.date,
        os: beacon_data.os.check(),
        beacons: beacon_id,
        r#type: beacon_data.r#type,
    })
}
