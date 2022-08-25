use crate::block::Block;
use serde::{Deserialize, Serialize};

/*
This is the chain struct
it will house all the blocks and some functions to do with the blocks/chain
*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chain {
    pub chain: Vec<Block>,
}
impl Chain {
    pub fn new() -> Self {
        //creates a new chain with the orgin block
        Self {
            chain: vec![Block::one_and_only()],
        }
    }

    //Gets the latest block in the chain
    pub fn latest(&self) -> Block {
        self.chain[self.chain.len() - 1].clone()
    }
    // get the latest block that does not have main = true
    pub fn get_latest_non_main(&self)->Block{
        for block in self.chain.clone(){
            if !block.main{
                return block
            }
        }
        panic!("how did he we get here")
    }
    // function to add block also makes sure its validated
    pub fn add_block(&mut self, b: Block) {
        if b.validate(self.latest()) || self.chain.len() == 0 {
            self.chain.push(b);
        }
    }
    //adds a block without validation
    pub unsafe fn _add(&mut self, b: Block) {
        self.chain.push(b);
    }
    //Compares 2 chains and returns the longest
    pub fn comp_chain(&mut self, c: Self) -> Self {
        if self.chain.len() >= c.chain.len() {
            return self.clone();
        }
        c.clone()
    }
    // validates the chain by checking all blocks
    #[allow(dead_code)]
    pub fn validate_chain(&self) -> bool {
        for x in 0..self.chain.len() {
            if x == 0 {
                continue;
            }
            let b1 = &self.chain[x - 1];
            let b2 = &self.chain[x];
            if !b2.validate(o!(b1)) {
                return false;
            }
        }
        true
    }
    // change background function 
    pub fn change_bg(&self){
        extern crate wallpaper;
        let block = self.get_latest_non_main();
        wallpaper::set_from_url(&block.bg_path).unwrap();
    }
}
