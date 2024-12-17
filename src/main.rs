pub mod coordinator;
pub mod database;
pub mod flody_console;
pub mod http;
pub mod kafka;
pub mod robot;
pub mod stomp;

#[tokio::main]
async fn main() {
    //coordinator::success_flow::run_success_flow(file_path).await
    coordinator::localization_flow::run_localization_flow_booting_and_no_job().await;
}
