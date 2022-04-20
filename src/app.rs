use crate::block::Block;
use chrono::Utc;
use hex;
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct App {
    pub blockchain: Vec<Block>,
}

impl App {
    /// Create blockchain, vector of blocks
    pub fn new() -> Self {
        Self { blockchain: vec![] }
    }

    /// Create the first block in the blockchain
    /// This is a special genesis block with nonce = 33
    pub fn genesis(&mut self) {
        let mut hasher = Sha256::new();
        let _genesis_hash = hasher.update(b"genesis");

        let genesis_block = Block {
            id: Uuid::new_v4(),
            hash: format!("{:X}", hasher.finalize()),
            previous_hash: "genesis".to_owned(),
            timestamp: Utc::now().timestamp(),
            data: "genesis".to_owned(),
            nonce: 33,
        };

        self.blockchain.push(genesis_block);
    }

    pub fn show_blockchain(&self) -> Vec<Block> {
        self.blockchain.clone()
    }
}
