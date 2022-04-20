// use uuid::Uuid;
// use serde::{Serialize, Deserialize};

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
}
