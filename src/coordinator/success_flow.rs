use crate::http::client::PantosHttpClient;
use crate::kafka::pantos_client::PantosKafkaClient;
use crate::stomp::pantos_client::run;

pub async fn run_success_flow(
    http_client: PantosHttpClient,
    kafka_client: PantosKafkaClient,
    //stomp_client: PantosStompClient,
) {
    let robot_uid = "dBK39Eak";
    let tracking_number = "trackingNumber-1";

    http_client.upload_excel().await;

    http_client.command_robot_loading().await;
    kafka_client.consume_desired_topic().await;
    run().await;

    http_client.identify_repesentative_invoice_barcode(tracking_number).await;

    http_client.start_work(robot_uid, tracking_number).await;
    kafka_client.consume_desired_topic().await;
}
