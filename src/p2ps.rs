use actix_web::{get, post, web::Bytes, App, HttpRequest, HttpResponse, HttpServer};
extern crate http;
const IP_DEF: &str = "10.61.";
use std::sync::Mutex;

use std::{collections::HashMap, thread};

use attohttpc;
use crate::{block::Block, chain::Chain};



#[macro_use]
use lazy_static::lazy_static;
lazy_static! {
    static ref CHAIN: Mutex<Chain> = Mutex::new(Chain::new());
}

use regex::Regex;
extern crate wallpaper;

#[get("/pong")]
async fn pong() -> HttpResponse {

    HttpResponse::Ok()
        .content_type("text/plain")
        .header("test", "sample")
        .body("Pong")
}


#[post("/chain")]
async fn get_chain(req: String) -> HttpResponse {
    let x:Chain = serde_json::from_str(&req).expect("msg");
    let big = CHAIN.lock().unwrap().comp_chain(x);
    CHAIN.lock().unwrap().chain = big.chain;
    
    let lat_blok = CHAIN.lock().unwrap().latest().clone();

    


    HttpResponse::Ok()
        .content_type("text/plain")
        .header("test", "sample")
        .body("Ok")
}



pub async fn server()-> std::io::Result<()>{
    HttpServer::new(|| App::new().service(pong))
        .bind(":8080")?
        .run()
        .await
}