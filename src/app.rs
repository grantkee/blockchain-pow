use crate::block::Block;
use uuid::Uuid;
use sha2::{Sha256, Digest};
use hex;
use chrono::Utc;

pub struct App {
    pub blockchain: Vec<Block>,
}

impl App {
    fn new() -> Self {
        Self { blockchain: vec![] }
    }

    fn genesis(&mut self) {
        let mut hasher = Sha256::new();
        let genesis_hash = hasher.update(b"genesis");

        let genesis_block = Block {
            id: Uuid::new_v4(),
            hash: String::from_utf8(hex::decode(hasher.finalize()).unwrap()).unwrap(),
            previous_hash: "genesis".to_owned(),
            timestamp: Utc::now().timestamp(),
            data: "genesis".to_owned(),
            nonce: 33,
        };

        self.blockchain.push(genesis_block);
    }
}
