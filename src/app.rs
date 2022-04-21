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
    
    //Debugging notes below. Above this line is the refactoring that works.
    //
    // println!("\nhash_to_binary()");
    // println!("r: {:?}\n", r); // r is the variable labeled "binary" above. This print would have been on line 15.
    // println!("std::str::from_utf8: {:?}\n", std::str::from_utf8(hash));
    // let res = match std::str::from_utf8(&hash) {
    //     Ok(slice) => slice,
    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // };
    
    // res.to_string()

    // Below is the output in the terminal.
    // 
    // note: this println came from mine_block() line 46.
    // &hash - [68, 208, 187, 236, 128, 31, 237, 19, 101, 200, 108, 218, 40, 14, 245, 218, 202, 122, 113, 12, 46, 45, 153, 239, 170, 143, 114, 216, 239, 98, 238, 62]

    // hash_to_binary()
    // r: "10001001101000010111011111011001000000011111111011011001111001011100100011011001101101010100011101111010111011010110010101111010111000111001011101011011001100111101111101010101000111111100101101100011101111110001011101110111110"
    
    // std::str::from_utf8: Err(Utf8Error { valid_up_to: 3, error_len: Some(2) })
    
    // thread 'main' panicked at 'Invalid UTF-8 sequence: invalid utf-8 sequence of 2 bytes from index 3', src/app.rs:20:19
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
}
