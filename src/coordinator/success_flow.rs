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

    let cloned_picking_ids = picking_ids_container.lock().unwrap().clone();
    let request_picking_ids: Vec<&str> = cloned_picking_ids.iter().map(String::as_str).collect();
    http_client.worker_arrived(&request_picking_ids).await;

    for picking_id in &request_picking_ids {
        let picking_dto = http_client.get_same_location_pickings(picking_id).await;

        for picking in picking_dto {
            http_client.complete_picking(&picking.picking_id).await;
        }
    }

    let assigned_pickings = http_client.get_all_assigned_pickings(robot_uid).await;
    
    http_client.get_total_unloadings().await;
    
    let mut workgroup_ids: Vec<&str> = vec![];
    for assigned_picking in &assigned_pickings {
        for picking_dto in &assigned_picking.pickings {
            workgroup_ids.push(&picking_dto.workgroup_id);
        }
    }
    http_client.complete_unloading(&workgroup_ids).await;
}
