#![recursion_limit = "256"]
use warp::{filters::path::param, reply::{Reply, Response}, Filter};
use std::{collections::HashMap, fs, string, sync::OnceLock};

#[tokio::main]
async fn main() {
    let text = fs::read_to_string("script.txt").unwrap();
    let splitting = text.split(&"-".repeat(42)).map(|t|  String::from(t)).collect::<Vec<String>>();
    let jatekleiras = splitting[0].clone();
    let hattertortenet = splitting[1].clone();
    let story = splitting[2].clone();
    let cards = story.split("#").map(|t| String::from(t)).collect::<Vec<String>>();

    //let staticstory: &'static str = Box::leak(Box::new(story));

    let port = 4040;
    println!("port is {}", port);
    let script = warp::path("script.js").and(warp::fs::file("script.js"));
    let home = warp::path::end().map(|| warp::reply::html(fs::read_to_string("html/index.html").unwrap()));
    let help = warp::path::end().map(|| warp::reply::html(fs::read_to_string("html/help.html").unwrap()));
    let tortenet = warp::path("tortenet").map(move || warp::reply::html(String::from("asdf".replace("asdf", &story))));
    let cards = warp::path!("card"/ usize).map(move |card: usize|
        warp::reply::html(cards[card].clone())
    );


    let routes = home.or(help).or(tortenet).or(cards)
    .or(script);
    warp::serve(routes).run(([0,0,0,0], port)).await;
}