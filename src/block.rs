use sha2::{Sha256, Digest}; 
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone,Debug,Serialize, Deserialize)]
pub struct Block{
    pub bg_path: String,
    pub time: i64,
    pub hash: String,
    pub prev_hash: String,
    pub main: bool
}
impl Block{
    pub fn one_and_only()->Self{
        Self { bg_path: s!("first"), time: 1661244907, hash: s!("3e2eed92b1223f9fd4f9bcc1adbe0e00b0e76f827d5eb6298bcfb48c555699c1"), prev_hash: s!("first"),main: true }
    }
    pub fn validate(&self, b: Self)->bool{
        self.prev_hash == b.hash
    }
    fn create_hash(a:String, b:i64)->String{

        let mut hasher = Sha256::new();
        hasher.update(format!("{},{}",a,b).as_bytes());

        // read hash digest and consume hasher
        let result = hasher.finalize();
        hex::encode(result)
    }
    pub fn new_block(path: String, maintance: bool,prev_block: Block)->Self{

        let hsh = Block::create_hash(path.clone(),Utc::now().timestamp());

        Self { bg_path: path, time: Utc::now().timestamp(), hash: hsh, prev_hash: prev_block.hash, main: maintance }
    }

}