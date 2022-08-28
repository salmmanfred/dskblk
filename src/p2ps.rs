/*
    TODO: Need to fix the computer not being able to send chain some of the times
    TODO: Need to make sure that there is only one chain
    TODO: Fix so that computer that has been on longer can send chain back not just connecting computer

*/



use actix_web::{get, post, App, HttpRequest, HttpResponse, HttpServer};
extern crate http;
const IP_DEF: &str = "10.61.";
//const IP_DEF: &str = "192.168.";

use std::sync::Mutex;
//use serde::{Deserialize, Serialize};

use std::thread;

use crate::{block::Block, chain::Chain};
use attohttpc;

//#[macro_use]

/*
Lazy_static is a global variable like const or static however it 
instead of defining the content of the variables at compile time does it at 
runtime.

*/
use lazy_static::lazy_static;
lazy_static! {
    static ref CHAIN: Mutex<Chain> = Mutex::new(Chain::new());
    static ref NETWORK: Mutex<Network> = Mutex::new(Network::new());

}


extern crate wallpaper;
// the different destinations

// this is just a test destination
#[get("/pong")]
async fn pong() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .header("test", "sample")
        .body("Pong")
}

//To share chains between computers this web page is used
#[post("/chain")]
async fn get_chain(req: String) -> HttpResponse {
    let x: Chain = serde_json::from_str(&req).expect("msg");
    println!("recived chain");
    if x.validate_chain(){
        let big = CHAIN.lock().unwrap().comp_chain(x);
        CHAIN.lock().unwrap().chain = big.chain;
    
        CHAIN.lock().unwrap().change_bg();
    }

    



    HttpResponse::Ok()
        .content_type("text/plain")
        .header("test", "sample")
        .body("Ok")
}

//recive new blocks
#[post("/block")]
async fn new_block(req: String) -> HttpResponse {
    println!("recived block");
    let x: Block = serde_json::from_str(&req).expect("msg");
    CHAIN.lock().unwrap().add_block(x);
    
    CHAIN.lock().unwrap().change_bg();
    

    

    HttpResponse::Ok()
        .content_type("text/plain")
        .header("test", "sample")
        .body("Ok")
}
// used to find new users
#[get("/find")]
async fn find(req: HttpRequest) -> HttpResponse {
    
    if let Some(val) = req.peer_addr() {
        //  println!("Address {:?}", val.ip());
        println!("New user has joined!");
        NETWORK.lock().unwrap().add_user(val.ip().to_string());
    };

    

    HttpResponse::Ok()
        .content_type("text/plain")
        .header("test", "sample")
        .body("Ok")
}


// starts the server
pub async fn server() -> std::io::Result<()> {
    //sweeps the network to find users
    sweep();
    sweep();
    NETWORK.lock().unwrap().send_chain();
    // spawns user input thread to be able to get inputs
    thread::spawn(||{

        loop{
            let inp = get_s(">");
            let inp = inp.split(" ").collect::<Vec<&str>>().clone();
            match inp[0]{
                "bg" =>{
                    let prvblock = CHAIN.lock().unwrap().latest();
                    let b = Block::new_block(s!(inp[1]), false, prvblock);
                    NETWORK.lock().unwrap().new_block(b);

                    println!("sent");

                }
                "bb" =>{
                    let a = NETWORK.lock().unwrap().users.clone(); // print it out 
                    println!("{:#?}",a);
                }
                "chain" => {
                    let a = CHAIN.lock().unwrap().chain.clone(); // print it out 
                    println!("{:#?}",a);
                }

                _=>{

                }
            }
        }
    });

    // starts the server 
    HttpServer::new(|| App::new().service(pong).service(get_chain).service(new_block).service(find))
        .bind(":3000")?
        .run()
        .await
}
// I stole this function but it gets the input from the user
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
// the network 
/*
The network holds all users and user based functions

Users: Vector(array) of all users ips in string format

*/
struct Network{
    users: Vec<String>
}
impl Network{
    // create a new network
    pub fn new()->Self{
        Self { users: Vec::new() }
    }
    // add a new user and makes sure it does not get added twice
    pub fn add_user(&mut self,ip: String){
        for x in self.users.clone(){
            if x == ip{
                return 
            }
        }
        self.users.push(ip)
    }
    // remove offending users 
    fn remove(&mut self, ip: String){
        for (i,x) in self.users.clone().iter().enumerate(){
            if x == &ip{
                self.users.remove(i);
            }
        }
    }
    //send a new block to all users 
    pub fn new_block(&mut self, b: Block){
        CHAIN.lock().unwrap().add_block(b.clone());
        for x in self.users.clone(){
            match attohttpc::post(format!("http://{}:3000/block",x,))
            .header("downloading", "file")
            .text(serde_json::to_string(&b).unwrap())
            .send(){
                Err(_) =>{
                    // kick the person because they are no longer active 
                    self.remove(x);
                }
                Ok(_) =>{
                    //ok
                }
            }; 
        }
    }
    pub fn send_chain(&self){
        let c = CHAIN.lock().unwrap().clone();

        for x in self.users.clone(){
            println!("Sending chain to user... {}",x);
            attohttpc::post(format!("http://{}:3000/chain",x ))
            .header("sending", "file")
            .text(serde_json::to_string(&c).unwrap())
            .send().unwrap();
            println!("sent");
        }
    }
}

// sweeps the internet for users 
fn sweep() {
    for y in 65..100 {
        // let mut thrs: Vec<JoinHandle<()>> = Vec::new();
        for x in 0..255 {
            thread::spawn(move || {
                match attohttpc::get(format!("http://{}{}.{}:3000/find", IP_DEF, y, x))
                    .header("downloading", "file")
                    .send()
                {
                    Err(_) => {
                        // println!("{:#?}", a);
                        // println!("http://192.168.68.{}:3000/find", x);
                    }
                    Ok(_) => {
                        //println!("{:#?}", a);
                        NETWORK.lock().unwrap().add_user(format!("{}{}.{}", IP_DEF, y, x));
                       // add_ip(format!("{}{}.{}", IP_DEF, y, x));

                        // println!("http://192.168.68.{}:3000/find", x);
                        println!("Found user");


                        
                    }
                }
            });
            //   thrs.push(thr);
        }
    }
    println!("ok");
}
