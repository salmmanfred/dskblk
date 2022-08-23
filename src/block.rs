use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/*
This is the Block

bg_path: the path to the picture
time: UTC unix time
hash: SHA256 hash of bgpath and time
prev_hash: Hash of the previous block
main: if the block even has a background or is just for other things

*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub bg_path: String,
    pub time: i64,
    pub hash: String,
    pub prev_hash: String,
    pub main: bool,
}
impl Block {
    //"Genesis" block
    pub fn one_and_only() -> Self {
        Self {
            bg_path: s!("first"),
            time: 1661244907,
            hash: s!("3e2eed92b1223f9fd4f9bcc1adbe0e00b0e76f827d5eb6298bcfb48c555699c1"),
            prev_hash: s!("first"),
            main: true,
        }
    }
    //Validates that the block has the correct prev_hash
    pub fn validate(&self, b: Self) -> bool {
        self.prev_hash == b.hash
    }

    // Creates the hash
    fn create_hash(a: String, b: i64) -> String {
        // create the hasher
        let mut hasher = Sha256::new();

        //add the text that needs to be hashed
        hasher.update(format!("{},{}", a, b).as_bytes());

        // read hash digest and consume hasher
        let result = hasher.finalize();
        //returns the hash in String format
        hex::encode(result)
    }
    // creates a new block
    pub fn new_block(path: String, maintance: bool, prev_block: Block) -> Self {
        //gets the hash
        let hsh = Block::create_hash(path.clone(), Utc::now().timestamp());
        //Creates the block and returns it
        Self {
            bg_path: path,
            time: Utc::now().timestamp(),
            hash: hsh,
            prev_hash: prev_block.hash,
            main: maintance,
        }
    }
}
