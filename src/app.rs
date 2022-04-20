use crate::block::Block;
use chrono::Utc;
use log::{error, warn};
use sha2::{Digest, Sha256};
use uuid::Uuid;

const REQUIRED_PREFIX: &str = "000";

async fn hash_to_binary(hash: &[u8]) -> String {
    std::str::from_utf8(hash).unwrap().to_string()
}

#[derive(Clone, Debug)]
pub struct App {
    blockchain: Vec<Block>,
}

impl App {
    /// Create blockchain, vector of blocks
    pub async fn new() -> Self {
        Self { blockchain: vec![] }
    }

    /// Create the first block in the blockchain
    /// This is a special genesis block with nonce = 33
    pub async fn genesis(&mut self) {
        let mut hasher = Sha256::new();
        let _genesis_hash = hasher.update(b"genesis");

        let genesis_block = Block {
            id: Uuid::new_v4(),
            position: 0,
            hash: format!("{:X}", hasher.finalize()),
            previous_hash: "genesis".to_owned(),
            timestamp: Utc::now().timestamp(),
            data: "genesis".to_owned(),
            nonce: 33,
        };

        self.blockchain.push(genesis_block);
    }

    pub async fn get_blockchain(&self) -> Vec<Block> {
        self.blockchain.clone()
    }

    async fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blockchain.last().expect("blockchain is empty.");
        if self.verify_block(&block, latest_block).await {
            self.blockchain.push(block);
        } else {
            error!("block invalid - cannot add to blockchain");
        }
    }

    async fn verify_block(&self, block: &Block, latest_block: &Block) -> bool {
        let result = false;
        // check if new block is pointing to the last block in the blockchain
        if block.previous_hash != latest_block.hash {
            warn!(
                "block {} does not have the correct previous hash.",
                block.id
            );
        } else if !hash_to_binary(&hex::decode(&block.hash).expect("can't decode hash from hex")).await
            .starts_with(REQUIRED_PREFIX)
        {
            warn!("block {} has an invalid hash prefix", block.id)
        }
        true
    }
}
