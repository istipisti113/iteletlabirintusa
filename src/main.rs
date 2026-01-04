#![recursion_limit = "256"]
use serde::de::value::Error;
use warp::{filters::path::param, reply::{Reply, Response}, Filter};
use core::num;
use std::{collections::HashMap, fs, iter::Enumerate, string, sync::OnceLock};
use regex::Regex;

struct Ellenseg {
    nev: String,
    ugyesseg: i32,
    eletero: i32,
}

enum Potion {
    Agility,
    Health,
    Luck
}

struct Player {
    agility: i32,
    initialagility: i32,
    health: i32,
    initialhealth: i32,
    luck: i32,
    initialluck:i32,
    potion: Potion
}

#[tokio::main]
async fn main() {
    let text = fs::read_to_string("script.txt").unwrap();
    let splitting = text.split(&"-".repeat(42)).map(|t|  String::from(t)).collect::<Vec<String>>();
    let jatekleiras = splitting[0].clone();
    let hattertortenet = splitting[1].clone();
    let story = splitting[2].clone();
    let cards = story.split("#").map(|t| String::from(t.replace("\n", "<br>"))).collect::<Vec<String>>();

    //let staticstory: &'static str = Box::leak(Box::new(story));

    let port = 4040;
    println!("port is {}", port);
    let script = warp::path("script.js").and(warp::fs::file("script.js"));
    let jatekscript = warp::path("jatek.js").and(warp::fs::file("jatek.js"));
    let css = warp::path("index.css").and(warp::fs::file("html/index.css"));
    let szabalyokcss = warp::path("szabalyok.css").and(warp::fs::file("html/szabalyok.css"));
    let home = warp::path::end().map(|| warp::reply::html(fs::read_to_string("html/index.html").unwrap()));
    let help = warp::path::end().map(|| warp::reply::html(fs::read_to_string("html/help.html").unwrap()));
    let header = warp::path("header").map(|| warp::reply::html(fs::read_to_string("html/header.html").unwrap()));
    let footer = warp::path("footer").map(|| warp::reply::html(fs::read_to_string("html/footer.html").unwrap()));
    let harc = warp::path("harc").map(|| warp::reply::html(fs::read_to_string("html/harc.html").unwrap()));
    let leiras = warp::path("leiras").map(move || warp::reply::html("a".replace("a", &h3(&jatekleiras))));
    let tortenetszoveg = warp::path("tortenetszoveg").map(move || warp::reply::html(String::from("asdf".replace("asdf", &h3(&hattertortenet)))));
    let segedfile = warp::path("segedfile").map(||{
        warp::reply::json(&fs::read_to_string("db.json").unwrap().replace("\n", ""))
    });

    let tortenet = warp::path("tortenet").map(move || warp::reply::html(fs::read_to_string("html/hattertortenet.html").unwrap()));
    let szabalyok = warp::path("szabalyok").map(||warp::reply::html(fs::read_to_string("html/szabalyok.html").unwrap()));
    //let jatek = warp::path("jatek").map(|| warp::reply::html(fs::read_to_string("html/jatek.html").unwrap()));
    let jatek = warp::path!("jatek"/i32/i32/i32/String)
        .map(|agil: i32, health: i32, luck: i32, potion: String|{
            warp::reply::html(fs::read_to_string("html/jatek.html").unwrap()
                .replace("AGILITY", &agil.to_string())
                .replace("LUCK", &luck.to_string())
                .replace("HEALTH", &health.to_string())
                .replace("POTION", &potion)
            )
        });

    let cardspath = warp::path!("card"/ usize).map(move |card: usize| {
        //println!("{}", searchForNumber(&cards[card]).unwrap_or(vec![0]).iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", "));
        let button = fs::read_to_string("html/lapozas.html").unwrap();
        let mut buttons: String = searchForNumber(&cards[card]).unwrap_or(vec![0]).iter().map(|a| a.to_string()).collect::<Vec<String>>().iter().map(|oldal|  {
            button.replace("OLDAL", &oldal)
        }).collect::<Vec<String>>().join("<br>");
        match vaneharc(&cards[card]) {
            Ok(enemies) => {
                //println!("{}", enemies.iter().count());
                buttons = fs::read_to_string("html/harcbutton.html").unwrap().replace("ENEMIES", &( String::from("\'")+ 
                    &enemies.iter().map(|enemy|{
                        return String::from(&enemy.nev)+","+ &enemy.ugyesseg.to_string()+","+ &enemy.eletero.to_string();
                    }).collect::<Vec<String>>().join(";")+"\'")
                )+ "<br>" + &buttons;
            }
            Err(e) => {
                //println!("{}",e)
            }
        }
        warp::reply::html(fs::read_to_string("html/kartya.html").unwrap()
            .replace("SZOVEG", &cards[card])
            .replace(&(card.to_string()+"<br>"), &("<h2>".to_string()+&card.to_string()+"</h2>"))
            .replace("GOMBOK", &buttons)
        )
    });

    let routes = home.or(help).or(tortenet).or(cardspath).or(tortenetszoveg).or(leiras).or(szabalyok).or(jatek).or(segedfile).or(jatekscript).or(harc)
    .or(script).or(css).or(szabalyokcss).or(header).or(footer);
    warp::serve(routes).run(([0,0,0,0], port)).await;
}

fn h3(szoveg: &str) -> String {
    let mut returning  = String::from(szoveg).split("\n").map(|line| String::from(line)).collect::<Vec<String>>();
    for i in 0..returning.iter().count(){
        if returning[i].trim().chars().count()==0{continue;}
        if returning[i].trim().chars().all(|c| c == '='){
            returning[i-1]= String::from("<br><h3>")+&returning[i-1]+"</h3>";
            returning[i] = String::new();
        }
        if !returning[i].is_empty(){
            returning[i] += "<br>";
        }
    }
    returning.join("")
}

fn searchForNumber(szoveg: &str) -> Result<Vec<i32>, String>{
    let re = Regex::new(r"(\d{1,3})-r").unwrap();
    let mut szamok :Vec<i32> = vec![];
    for num in re.captures_iter(szoveg){
        szamok.push(num[1].parse::<i32>().unwrap());
    }
    if szamok.iter().count()!=0{return Ok(szamok);}
    return Err("Nincs szam".to_string());
}

fn vaneharc(szoveg: &str) -> Result<Vec<Ellenseg>, String>{
    //let re = Regex::new(r"<br><br>(.*)<br>ÜGYESSÉG (\d{1,2})<br>ÉLETERŐ (\d{1,3})").unwrap();
    let re = Regex::new(r"<br><br>(?s)(.+?)<br>\s*ÜGYESSÉG\s*(\d+)<br>\s*ÉLETERŐ\s*(\d+)").unwrap();
    let mut ellensegek: Vec<Ellenseg> = vec![];
    for enemy in re.captures_iter(szoveg){
        ellensegek.push(Ellenseg { nev: enemy[1].to_string(), ugyesseg: enemy[2].to_string().parse::<i32>().unwrap(), eletero: enemy[3].to_string().parse::<i32>().unwrap() });
        //println!("---{}, {}, {}---", &enemy[1], &enemy[2], &enemy[3]);
    }
    if ellensegek.iter().count()==0{return Err("nincs harc".to_string());}
    return Ok(ellensegek);
}
