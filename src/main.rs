use uuid::Uuid;
use serde::{Serialize, Deserialize};

pub struct App {
    pub blockchain: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: Uuid,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

fn main() {
    println!("Hello, world!");
}
