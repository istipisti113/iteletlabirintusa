#![recursion_limit = "256"]
use warp::{filters::path::param, reply::{Reply, Response}, Filter};
use std::{collections::HashMap, fs, string};

#[tokio::main]
async fn main() {
    let port = 4040;
    println!("port is {}", port);
    let home = warp::path::end().map(|| warp::reply::html(fs::read_to_string("index.html").unwrap()));
    let home2 = warp::path("index").map(|| warp::reply::html(fs::read_to_string("index.html").unwrap()));

    let routes = home2;
    warp::serve(routes).run(([0,0,0,0], port)).await;
}