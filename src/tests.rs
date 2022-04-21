use crate::app::App;
use crate::block::Block;
use uuid::Uuid;
use sha2::{Digest, Sha256};
use chrono::Utc;

async fn create_app() -> App {
    let mut app = App::new().await;
    app.genesis().await;
    app
}

// test for 
#[tokio::test]
async fn create_blockchain() {
    // ensure genesis block is present
    let app = create_app().await;
    assert!(app.get_blockchain().await.len() == 1)
}

// tests for try_add_block
#[tokio::test]
async fn wrong_previous_hash() {
    let mut app = create_app().await;

    // generate hashes
    let mut hasher = Sha256::new();
    hasher.update(b"wrong_hash");
    let prev_hash = hasher.finalize();
    let mut hasher2 = Sha256::new();
    hasher2.update(b"test");
    let next_hash = hasher2.finalize();

    // create dummy block
    let block = Block {
        id: Uuid::new_v4(),
        position: 1,
        hash: format!("{:X}", next_hash),
        previous_hash: format!("{:X}", prev_hash),
        timestamp: Utc::now().timestamp(),
        data: "test".to_owned(),
        nonce: 0,
    };

    let attempt = app.try_add_block(block).await;
    assert!(attempt == false)
}

#[tokio::test]
async fn wrong_proof_of_work() {
    let mut app = create_app().await;

    // generate hashes
    let mut hasher = Sha256::new();
    hasher.update(b"genesis");
    let prev_hash = hasher.finalize();
    let mut hasher2 = Sha256::new();
    hasher2.update(b"test");
    let next_hash = hasher2.finalize();

    // create dummy block
    let block = Block {
        id: Uuid::new_v4(),
        position: 1,
        hash: format!("{:X}", next_hash),
        previous_hash: format!("{:X}", prev_hash),
        timestamp: Utc::now().timestamp(),
        data: "test".to_owned(),
        nonce: 0,
    };

    let attempt = app.try_add_block(block).await;
    assert!(attempt == false)
}


#[tokio::test]
async fn new_block() {
    let block = Block::new(
        Uuid::new_v4(),
        1,
        "genesis".to_owned(),
        "test".to_owned()
    ).await;

    assert!(block.hash.is_empty() == false);
}

#[tokio::test]
async fn verify_chain() {
    let mut app = create_app().await;

    let block = Block::new(
        Uuid::new_v4(),
        1,
        "genesis".to_owned(),
        "test".to_owned()
    ).await;

    app.try_add_block(block).await;

    let chain_is_valid = app.verify_chain(&app.blockchain).await;

    assert!(chain_is_valid == true);
}
