use crate::app::App;

async fn create_app() -> App {
    let mut app = App::new().await;
    app.genesis().await;
    app
}

#[tokio::test]
async fn create_blockchain() {
    // ensure genesis block is present
    let app = create_app().await;
    assert!(app.get_blockchain().await.len() == 1)
}

#[tokio::test]
async fn unverified_block() {
    todo!()
}
