use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

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
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce,
    });

    let mut hasher = Sha256::new();
    hasher.update(hash.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
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
