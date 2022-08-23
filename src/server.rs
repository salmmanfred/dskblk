use actix_web::{get, post, web::Bytes, App, HttpRequest, HttpResponse, HttpServer};
extern crate http;
const IP_DEF: &str = "10.61.";
use std::sync::Mutex;

use std::{collections::HashMap, thread};

use attohttpc;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
extern crate wallpaper;
struct Users {
    ip: HashMap<usize, String>,
    ip_rev: HashMap<String, usize>,
}
impl Users {
    pub fn new() -> Self {
        Self {
            ip: HashMap::new(),
            ip_rev: HashMap::new(),
        }
    }
    pub fn send_message(&mut self, mess: &str) {
        // sends the messages to all Users ips
        for x in 0..self.ip.len() {
            let ip = self.ip.get(&x).unwrap();
            // println!("sending to: {}", ip);
            let _ = attohttpc::post(&format!("http://{}:8080/message", ip))
                .header("downloading", "file")
                .text(mess)
                .send();
        }
    }
    pub fn change_bg(&mut self, url: &str) {
        // changes the background for all Users
        for x in 0..self.ip.len() {
            let ip = self.ip.get(&x).unwrap();
            //  println!("sending to: {}", ip);
            let url = &url.to_string().replace("https://", "");
            println!("x{}x",url);
            let _ = attohttpc::post(&format!("http://{}:8080/bg", ip))
                .header("Settingbg", "file")
                .text(url)
                .send()
                .unwrap();
        }
    }
    pub fn ping(&mut self){
        let mut how_many = 0;
        for x in 0..self.ip.len(){
            let ip = self.ip.get(&x).unwrap();
            let x = attohttpc::get(&format!("http://{}:8080/pong", ip))
                .header("pong", "ping")
                //.header("val", Bytes::from(data.clone()))
                .send()
                .unwrap();
            match x.text(){
                Ok(a)=>{
                    match a.as_str(){
                        "Pong"=>{
                            println!("{}",a);
                            how_many += 1;

                        }
                        _=>{
                            
                        }
                    }
                }
                _=>{

                }
            }
        }
        if how_many >= self.ip.len(){
            println!("all responded");
        }else{
            println!("{} out of {} responded.", how_many, self.ip.len())
        }
    }
    #[allow(dead_code)]
    pub fn change_bgb(&mut self, data: Vec<u8>, name: &str) {
        for x in 0..self.ip.len() {
            let ip = self.ip.get(&x).unwrap();
            //   println!("sending to: {}", ip);,

            let x = attohttpc::post(&format!("http://{}:8080/bgb", ip))
                .header("Settingbg", name)
                //.header("val", Bytes::from(data.clone()))
                .bytes(Bytes::from(data.clone()))
                .send()
                .unwrap();
            println!("{:#?}", x)
        }
    }
}

lazy_static! {
    static ref USERS: Mutex<Users> = Mutex::new(Users::new());
}
lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(i\.imgur\.com.*\.(png|jpeg|jpg))").unwrap();
}
fn add_ip(ip: String) {
    let mut x = USERS.lock().unwrap();
    if !x.ip_rev.contains_key(&ip) {
        let id = x.ip.len();

        x.ip.insert(id.clone(), ip.clone());
        x.ip_rev.insert(ip, id.clone());
    }
}

#[get("/find")]
async fn find(req: HttpRequest) -> HttpResponse {
    // gets the local ip of the person looking for you
    // after this it puts it into the Users struct
    //
    if let Some(val) = req.peer_addr() {
        //  println!("Address {:?}", val.ip());
        println!("New user has joined!");
        add_ip(val.ip().to_string());
    };

    HttpResponse::Ok()
        .content_type("text/plain")
        .header("test", "sample")
        .body("hello")
}
#[post("/message")]
async fn message(req: String) -> HttpResponse {
    println!("Message: {}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .header("test", "sample")
        .body("Ok")
}

#[get("/pong")]
async fn pong() -> HttpResponse {

    HttpResponse::Ok()
        .content_type("text/plain")
        .header("test", "sample")
        .body("Pong")
}

#[post("/bg")]
async fn bg(req: String) -> HttpResponse {
    // downloads and sets the background
    wallpaper::set_from_url(&format!("http://{}", req.clone())).unwrap();

    /*  let pic = attohttpc::get(format!("http://{}", req.clone()))
        .header("downloading", "file")
        .send()
        .unwrap();

    let x = REGEX.captures(&req).unwrap();*/

    /*match &x[2] {
        "jpeg" => {
            openfile::write_file_bytes("1.jpeg", pic.bytes().unwrap()).unwrap();
            wallpaper::set_from_path("1.jpeg").unwrap();
            wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
        }
        "jpg" => {
            openfile::write_file_bytes("1.jpg", pic.bytes().unwrap()).unwrap();

            wallpaper::set_from_path("1.jpg").unwrap();
            wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
        }
        "png" => {
            openfile::write_file_bytes("1.png", pic.bytes().unwrap()).unwrap();
            wallpaper::set_from_path("1.png").unwrap();
            wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
        }
        _ => {}
    }*/

    HttpResponse::Ok()
        .content_type("text/plain")
        .header("test", "sample")
        .body("Ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    sweep();
    sweep();

    thread::spawn(move || loop {
        let inp = get_s(">");
        let inp = inp.split(" ").collect::<Vec<&str>>().clone();
        match inp[0].clone() {
            "users" => {
                println!("{:#?}", USERS.lock().unwrap().ip)
            }
            "ping"=>{
                USERS.lock().unwrap().ping();
            }
            "sweep" => sweep(),
            "bg" => {
                let mut x = USERS.lock().unwrap();
                println!("{}",inp[1]);
                x.change_bg(inp[1]);
                //(i\.imgur\.com.*\.(png|jpeg|jpg))
              /* let re = Regex::new(r"(i\.imgur\.com.*\.(png|jpeg|jpg))").unwrap();
                match re.captures(inp[1]) {
                    Some(a) => {
                       
                    }
                    None => {
                        println!("Invalid url try again")
                    }
                } */

                // if inp[1]
            }

            _ => {
                USERS.lock().unwrap().send_message(&inp.join(" "));
            }
        }
    });

    HttpServer::new(|| App::new().service(bg).service(pong).service(find).service(message))
        .bind(":8080")?
        .run()
        .await
}
fn get_s(ss: &str) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{}", ss);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}
fn sweep() {
    for y in 65..100 {
        // let mut thrs: Vec<JoinHandle<()>> = Vec::new();
        for x in 0..255 {
            thread::spawn(move || {
                match attohttpc::get(format!("http://{}{}.{}:8080/find", IP_DEF, y, x))
                    .header("downloading", "file")
                    .send()
                {
                    Err(_) => {
                        // println!("{:#?}", a);
                        // println!("http://192.168.68.{}:8080/find", x);
                    }
                    Ok(_) => {
                        //println!("{:#?}", a);
                        add_ip(format!("{}{}.{}", IP_DEF, y, x));

                        // println!("http://192.168.68.{}:8080/find", x);
                        println!("Found user");
                    }
                }
            });
            //   thrs.push(thr);
        }
        /*  for x in thrs{
            x.join().unwrap();
        }*/
        // println!("done: {}",y);
    }
}
