use crate::app::App;

async fn create_app() -> App {
    let mut app = App::new();
    app.genesis();
    app
}

#[tokio::test]
async fn create_blockchain() {
    // ensure genesis block is present
    let app = create_app().await;
    assert!(app.get_blockchain().len() == 1)
}
