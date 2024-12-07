use crate::http::client::PantosHttpClient;
use crate::kafka::pantos_client::PantosKafkaClient;

pub async fn run_success_flow(http_client: PantosHttpClient, kafka_client: PantosKafkaClient) {
    http_client.upload_excel().await;
    http_client.command_robot_loading().await;
    kafka_client.received_command_move_to_loading_zone().await;
}
