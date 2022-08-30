mod block;

mod chain;

mod p2ps;

#[macro_use]
extern crate handy_macros;

#[actix_web::main]
async fn main() {
    
    
    p2ps::server().await.unwrap();
}
