// use uuid::Uuid;
// use serde::{Serialize, Deserialize};

mod app;
mod block;
#[cfg(test)]
mod tests;

fn main() {
    let mut app = app::App::new();
    let _genesis = app.genesis();
    let blockchain_of_one = app.get_blockchain();

    for block in blockchain_of_one.iter() {
        println!("{:?}", block)
    }
}
