pub mod coordinator;
pub mod database;
pub mod flody_console;
pub mod http;
pub mod kafka;
pub mod robot;
pub mod stomp;

#[tokio::main]
async fn main() {
    let file_path = "./resources/wave-long";
    database::db::setup(file_path).await;

    coordinator::success_flow::run_success_flow(file_path).await
}
