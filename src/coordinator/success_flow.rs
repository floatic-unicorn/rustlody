use std::sync::{Arc, Mutex};

use crate::http::client::PantosHttpClient;
use crate::kafka::pantos_client::PantosKafkaClient;

pub async fn run_success_flow(
    http_client: PantosHttpClient,
    kafka_client: PantosKafkaClient,
    picking_ids_container: Arc<Mutex<Vec<String>>>,
) {
    let robot_uid = "dBK39Eak";
    let tracking_number = "trackingNumber-1";

    http_client.upload_excel().await;

    http_client.command_robot_loading().await;
    kafka_client.consume_desired_topic().await;

    http_client
        .identify_repesentative_invoice_barcode(tracking_number)
        .await;

    http_client.start_work(robot_uid, tracking_number).await;
    kafka_client.consume_desired_topic().await;

    println!("### picking_ids = {:?}", picking_ids_container);
}
