use crate::block::Block;
use serde::{Deserialize, Serialize};
#[derive(Clone,Debug,Serialize, Deserialize)]
pub struct Chain{
    pub chain: Vec<Block>,
    
}
impl Chain{
    pub fn new()->Self{
        
        Self { 
            chain: vec![Block::one_and_only()]
        }
    }
    pub fn latest(&self)->Block{
        
        self.chain[self.chain.len()-1].clone()
    }
    pub fn add_block(&mut self, b: Block){
        if b.validate(self.latest()) || self.chain.len() == 0{
            self.chain.push(b);
        }
    }
    pub unsafe fn _add(&mut self, b: Block){
        self.chain.push(b);

    }
    pub fn comp_chain(&mut self, c: Self) ->Self{
        if self.chain.len() >= c.chain.len(){
            return self.clone()
        }
        c.clone()
    }
    #[allow(dead_code)]
    pub fn validate_chain(&mut self)->bool{
        for x in 0..self.chain.len(){
            if x == 0 {
                continue;
            }
            let b1 = &self.chain[x-1];
            let b2 = &self.chain[x];
            if !b2.validate(o!(b1)) {
                return false;
            }
        }
        true
    }
}