use chrono::Utc;
use log::info;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::app::{hash_to_binary, REQUIRED_PREFIX};

pub async fn calc_hash(
    id: Uuid,
    position: usize,
    previous_hash: &str,
    timestamp: i64,
    data: &str,
    nonce: u64,
) -> Vec<u8> {
    let hash = serde_json::json!({
        "id": id,
        "position": position,
        "previous_hash": previous_hash,
        "timestamp": timestamp,
        "data": data,
        "nonce": nonce,
    });

    let mut hasher = Sha256::new();
    hasher.update(hash.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

pub async fn mine_block(
    id: Uuid,
    position: usize,
    previous_hash: &str,
    timestamp: i64,
    data: &str,
) -> (u64, String) {
    info!("mining block...");
    let mut nonce = 0;

    loop {
        if nonce % 100000 == 0 {
            info!("nonce: {}", nonce);
        }
        let hash = calc_hash(id, position, previous_hash, timestamp, data, nonce).await;
        let binary_hash = hash_to_binary(&hash).await;
        if binary_hash.starts_with(REQUIRED_PREFIX) {
            info!(
                "block successfully mined:\nnonce: {}\nhash: {}\nbinary: {}\n",
                nonce,
                hex::encode(&hash),
                binary_hash,
            );
            return (nonce, hex::encode(hash));
        }
        nonce += 1;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: Uuid,
    pub position: usize,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub async fn new(id: Uuid, position: usize, previous_hash: String, data: String) -> Self {
        let time_now = Utc::now().timestamp();
        let (nonce, hash) = mine_block(id, position, &previous_hash, time_now, &data).await;
        Self {
            id,
            position,
            hash,
            previous_hash,
            timestamp: time_now,
            data,
            nonce,
        }
    }
}
