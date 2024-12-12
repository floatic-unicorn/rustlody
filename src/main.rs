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
    let file_path = "./resources/wave-long";

    let picking_ids_container = Arc::new(Mutex::new(vec![]));
    let latest_status_container = Arc::new(Mutex::new("".to_string()));
    stomp::pantos_client::PantosStompClient::init(
        latest_status_container.clone(),
        picking_ids_container.clone(),
    ).await;

    database::db::setup(file_path).await;

    coordinator::success_flow::run_success_flow(
        pantos_http_client,
        kafka_client,
        latest_status_container,
        picking_ids_container,
        file_path,
    )
    .await
}
