use axum::{extract, prelude::*, response};
use hyper;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = route("/ping", post(ping)).route("/beacon", post(get_beacon_data));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    // ここunwrapしないでちゃんとエラー見る
}

#[derive(Deserialize)]
struct Ping {
    count: i64,
}

#[derive(Serialize)]
struct Pong {
    count: i64,
}

#[derive(Deserialize)]
struct BeaconData {
    date: String,
    os: OSType,
    r#type: String,
    beacons: Vec<Beacon>,
}

#[derive(Deserialize)]
struct Beacon {
    uuid: String,
    major: i64,
    minor: i64,
    rssi: f64,
    distance: f64,
}

#[derive(Deserialize)]
enum OSType {
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
struct BeaconResult {
    beacon: Vec<String>,
    response: String,
    hoge: String,
}

async fn get_beacon_data(
    extract::Json(get_beacon_data): extract::Json<BeaconData>,
) -> response::Json<BeaconResult> {
    let result: String = get_beacon_data.os.check();

    if get_beacon_data.beacons.len() == 0 {
        return response::Json(BeaconResult {
            response: result,
            beacon: vec!["".to_string()],
            hoge: "a".to_string(),
        });
    }

    // for _item in &get_beacon_data.beacons {
    //     let _uuid = (_item.uuid).to_string();
    //     let _major = (_item.major).to_string();
    //     let _minor = (_item.major).to_string();
    //     let _beacon_id = _uuid + ":" + &_major + ":" + &_minor;
    // }

    let beacon_id = get_beacon_data
        .beacons
        .iter()
        .map(|item| {
            (&item.uuid).to_string() + ":" + &item.major.to_string() + ":" + &item.minor.to_string()
        })
        .collect::<Vec<String>>();

    response::Json(BeaconResult {
        beacon: beacon_id,
        response: result,
        hoge: "a".to_string(),
    })
}

async fn ping(extract::Json(ping): extract::Json<Ping>) -> response::Json<Pong> {
    response::Json(Pong {
        count: ping.count + 1,
    })
}
