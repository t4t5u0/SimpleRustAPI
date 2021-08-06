mod beacon;

use axum::{extract, prelude::*, response};
use beacon::json_parse::get_beacon_data;
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

async fn ping(extract::Json(ping): extract::Json<Ping>) -> response::Json<Pong> {
    response::Json(Pong {
        count: ping.count + 1,
    })
}
