use std::sync::{Arc, Mutex};

pub mod coordinator;
pub mod database;
pub mod http;
pub mod kafka;
pub mod stomp;

#[tokio::main]
async fn main() {
    let pantos_http_client = http::client::PantosHttpClient::new();
    let kafka_client = kafka::pantos_client::PantosKafkaClient::new();

    let picking_ids_container = Arc::new(Mutex::new(vec![]));
    stomp::pantos_client::PantosStompClient::init(picking_ids_container.clone()).await;

    database::db::setup().await;

    coordinator::success_flow::run_success_flow(
        pantos_http_client,
        kafka_client,
        picking_ids_container,
    )
    .await
}
