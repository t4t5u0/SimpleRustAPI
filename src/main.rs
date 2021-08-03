use axum::{extract, prelude::*, response};
use hyper;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // let app = route("/ping", post(ping));
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // hyper::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    let beacon_app = route("/beacon", post(get_beacon_data));
    let beacon_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&beacon_addr)
        .serve(beacon_app.into_make_service())
        .await
        .unwrap();
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
    major: i64,
    minor: i64,
    rssi: i64,
    distance: i64,
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
    response: String,
    hoge: String,
}

async fn get_beacon_data(
    extract::Json(get_beacon_data): extract::Json<BeaconData>,
) -> response::Json<BeaconResult> {
    let result: String = get_beacon_data.os.check();
    response::Json(BeaconResult {
        response: result,
        hoge: "a".to_string(),
    })
}

async fn ping(extract::Json(ping): extract::Json<Ping>) -> response::Json<Pong> {
    response::Json(Pong {
        count: ping.count + 1,
    })
}
