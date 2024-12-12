use std::sync::{Arc, Mutex};

pub mod coordinator;
pub mod database;
pub mod flody_console;
pub mod http;
pub mod kafka;
pub mod robot;
pub mod stomp;

#[tokio::main]
async fn main() {
    let flody_console = flody_console::console::FlodyConsole::new();
    let dlody = robot::dlody::Dlody::new();
    let file_path = "./resources/wave-long";

    let picking_ids_container = Arc::new(Mutex::new(vec![]));
    let latest_status_container = Arc::new(Mutex::new("".to_string()));
    stomp::pantos_client::PantosStompClient::init(
        latest_status_container.clone(),
        picking_ids_container.clone(),
    )
    .await;

    database::db::setup(file_path).await;

    coordinator::success_flow::run_success_flow(
        flody_console,
        dlody,
        latest_status_container,
        picking_ids_container,
        file_path,
    )
    .await
}
