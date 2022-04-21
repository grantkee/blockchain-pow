use crate::block::{calc_hash, Block};
use chrono::Utc;
use log::{error, warn};
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub const REQUIRED_PREFIX: &str = "00";

pub async fn hash_to_binary(hash: &[u8]) -> String {
    let mut binary: String = String::default();
    for c in hash {
        binary.push_str(&format!("{:b}", c))
    }
    
    binary
}

#[derive(Clone, Debug)]
pub struct App {
    pub blockchain: Vec<Block>,
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

    pub async fn try_add_block(&mut self, block: Block) -> bool {
        let latest_block = self.blockchain.last().expect("blockchain is empty.");
        let mut result = false;
        if self.verify_block(&block, latest_block).await {
            self.blockchain.push(block);
            result = true;
        } else {
            error!("block invalid - cannot add to blockchain");
        }
        result
    }

    /// verify block checks hash value, proof-of-work, block position, and encoded hash
    pub async fn verify_block(&self, block: &Block, last_block: &Block) -> bool {
        let mut result = true;
        // check if new block is pointing to the last block in the blockchain
        if block.previous_hash != last_block.hash {
            warn!(
                "block {} does not have the correct previous hash.",
                block.id
            );
            result = false;
        }

        // ensure proof-of-work is valid
        if !hash_to_binary(&hex::decode(&block.hash).expect("can't decode hash from hex"))
            .await
            .starts_with(REQUIRED_PREFIX)
        {
            warn!("block {} has an invalid hash prefix", block.id);
            result = false;
        }

        // ensure correct block position
        if block.position != last_block.position + 1 {
            warn!(
                "block {} does not have the correct position: {} compared to the last block, position: {}", block.id, block.position, last_block.position
            );
            result = false;
        }

        // ensure calculated hashes match
        if hex::encode(
            calc_hash(
                block.id,
                block.position,
                &block.previous_hash,
                block.timestamp,
                &block.data,
                block.nonce,
            )
            .await,
        ) != block.hash
        {
            warn!("block {} has an invalid hash.", block.id);
            result = false;
        }
        result
    }

    pub async fn verify_chain(&self, blockchain: &[Block]) -> bool {
        // skip genesis block
        for i in 1..blockchain.len() {
            let first_block = blockchain.get(i - 1).expect("error with first block");
            let second_block = blockchain.get(i).expect("error with second block");
            if !self.verify_block(second_block, first_block).await {
                return false;
            }
        }

        true
    }
}
