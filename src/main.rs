use uuid::Uuid;
use sha2::{Digest, Sha256};
use crate::block::Block;

mod app;
mod block;
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    let mut app = app::App::new().await;
    let _genesis = app.genesis().await;
    let blockchain_of_one = app.get_blockchain();

    for block in blockchain_of_one.await.iter() {
        println!("{:?}", block)
    }

    let mut hasher = Sha256::new();
    hasher.update(b"genesis");
    let prev_hash = hasher.finalize();

    let _block = Block::new(
        Uuid::new_v4(),
        1,
        format!("{:X}", prev_hash),
        "test".to_owned()
    ).await;
}
