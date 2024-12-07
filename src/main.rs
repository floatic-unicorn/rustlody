pub mod coordinator;
pub mod database;
pub mod http;
pub mod kafka;
pub mod stomp;

#[tokio::main]
async fn main() {
    let pantos_http_client = http::client::PantosHttpClient::new();
    let kafka_client = kafka::pantos_client::PantosKafkaClient::new();

    database::db::setup().await;

    coordinator::success_flow::run_success_flow(pantos_http_client, kafka_client).await
}
