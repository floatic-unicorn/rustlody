use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

use crate::http::client::PantosHttpClient;
use crate::kafka::pantos_client::PantosKafkaClient;

pub async fn run_success_flow(
    http_client: PantosHttpClient,
    kafka_client: PantosKafkaClient,
    latest_status_container: Arc<Mutex<String>>,
    picking_ids_container: Arc<Mutex<Vec<Vec<String>>>>,
    wave_file_path: &str,
) {
    println!("\n\n===START SUCCESS FLOW TEST===\n\n");

    let robot_uid = "dBK39Eak";
    let tracking_number = "6078917960521";  //must be one of the number in wave_file_path file 
    
    http_client.upload_excel(wave_file_path).await;

    /* start loading */
    http_client.command_robot_loading().await;
    kafka_client.consume_desired_topic().await;
    kafka_client.publish_started_loading(robot_uid).await;
    kafka_client.publish_loading(robot_uid).await;

    let _ = http_client
        .identify_repesentative_invoice_barcode(tracking_number)
        .await;

    /* start picking */
    http_client.start_work(robot_uid, tracking_number).await;
    let mut desired_robot_status = kafka_client.consume_desired_topic().await;
    assert!(desired_robot_status == "PICKING");

    while desired_robot_status != "UNLOADING" {
        kafka_client.publish_started_picking(robot_uid).await;
        {
            let mut latest_status = latest_status_container.lock().unwrap().clone();
            while latest_status != "MOVING_FOR_PICKING" {
                sleep(Duration::from_millis(100)).await;
                latest_status = latest_status_container.lock().unwrap().clone();
            }
        }

        kafka_client.publish_picking(robot_uid).await;
        {
            let mut latest_status = latest_status_container.lock().unwrap().clone();
            while latest_status != "WAITING_WORKER_TO_PICK" {
                sleep(Duration::from_millis(100)).await;
                latest_status = latest_status_container.lock().unwrap().clone();
            }
        }

        let request_picking_ids: Vec<String> = {
            let mut cloned_picking_ids_container = picking_ids_container.lock().unwrap();
            let received_picking_ids = cloned_picking_ids_container.remove(0);
            assert!(received_picking_ids.len() != 0);
            received_picking_ids 
        };

        http_client.worker_arrived(&request_picking_ids).await;
        {
            let mut latest_status = latest_status_container.lock().unwrap().clone();
            while latest_status != "PICKING" {
                sleep(Duration::from_millis(100)).await;
                latest_status = latest_status_container.lock().unwrap().clone();
            }
        }

        for picking_id in &request_picking_ids {
            let _ = http_client.get_same_location_pickings(picking_id).await;

            http_client.complete_picking(picking_id).await;
        }
        desired_robot_status = kafka_client.consume_desired_topic().await;
    }

    /* wait until f/c receives unloading are done */    
    kafka_client.publish_started_unloading(robot_uid).await;
    kafka_client.publish_unloading(robot_uid).await;

    let mut latest_fc_status = latest_status_container.lock().unwrap().clone();
    while latest_fc_status != "UNLOADING" {
        println!("\n[RUSTLODY] pickings not done, proceeding to sleep\n");

        latest_fc_status = latest_status_container.lock().unwrap().clone();
        sleep(Duration::from_millis(500)).await;
    }

    /* start unloading */
    let total_unloadings = http_client.get_total_unloadings().await;

    let mut workgroup_ids: Vec<&str> = vec![];
    for unloading in &total_unloadings.in_progresses {
        workgroup_ids.push(&unloading.workgroup_id);
    }
    http_client.complete_unloading(&workgroup_ids).await;

    kafka_client.consume_desired_topic().await;

    /* Simulation finished; test states */
    let final_fc_status = latest_status_container.lock().unwrap().clone();
    assert!(final_fc_status == "MOVING_FOR_LOADING".to_string());

    println!("\n\n===END SUCCESS FLOW TEST===\n\n");
}
