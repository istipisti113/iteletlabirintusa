#![recursion_limit = "256"]
use warp::{filters::path::param, reply::{Reply, Response}, Filter};
use std::{collections::HashMap, fs, string};

#[tokio::main]
async fn main() {
    let port = 4040;
    println!("port is {}", port);
    let home = warp::path::end().map(|| warp::reply::html(fs::read_to_string("html/index.html").unwrap()));
    let help = warp::path::end().map(|| warp::reply::html(fs::read_to_string("html/help.html").unwrap()));

    let text = fs::read_to_string("script.txt").unwrap();
    let splitting = text.split(&"-".repeat(42)).collect::<Vec<&str>>();
    let jatekleiras = splitting[0];
    let hattertortenet = splitting[1];
    let story = splitting[2];

    let routes = home.or(help);
    warp::serve(routes).run(([0,0,0,0], port)).await;
}