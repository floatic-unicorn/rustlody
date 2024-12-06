pub mod http;
pub mod stomp;
pub mod kafka;
pub mod coordinator;
pub mod database;

#[tokio::main]
async fn main() {
    let pantos_http_client = http::client::PantosHttpClient::new();

    database::db::setup().await;

    coordinator::success_flow::run_success_flow(pantos_http_client).await
}
