mod block;
use crate::block::Block;
mod chain;

mod p2ps;

#[macro_use]
extern crate handy_macros;

#[actix_web::main]
async fn main() {
    // *TODO: main function needs cleaning upp and adding of a thread
    println!("Hello, world!");

    let mut main_chain = chain::Chain::new();

    main_chain.add_block(Block::new_block(
        s!("google.com"),
        false,
        main_chain.latest(),
    ));
    println!("{:#?}", main_chain);
    p2ps::server().await;
}
