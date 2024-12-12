use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

use crate::flody_console::console::FlodyConsole;
use crate::http::pantos_client::PantosHttpClient;
use crate::kafka::pantos_client::PantosKafkaClient;
use crate::robot::dlody::Dlody;

pub async fn run_success_flow(
    flody_console: FlodyConsole,
    dlody: Dlody,
    latest_status_container: Arc<Mutex<String>>,
    picking_ids_container: Arc<Mutex<Vec<Vec<String>>>>,
    wave_file_path: &str,
) {
    println!("\n\n===START SUCCESS FLOW TEST===\n\n");

    let robot_uid = "dBK39Eak";
    let tracking_number = "6078917960521"; //must be one of the number in wave_file_path file

    flody_console.upload_excel(wave_file_path).await;
    sleep(Duration::from_millis(50)).await;

    /* start loading */
    flody_console.command_robot_loading().await;
    dlody.consume_desired_topic().await;
    dlody.publish_started_loading(robot_uid).await;
    dlody.publish_loading(robot_uid).await;

    let _ = flody_console
        .identify_repesentative_invoice_barcode(tracking_number)
        .await;

    /* start picking */
    flody_console.start_work(robot_uid, tracking_number).await;
    let mut desired_robot_status = dlody.consume_desired_topic().await;
    assert!(desired_robot_status == "PICKING");

    while desired_robot_status != "UNLOADING" {
        dlody.publish_picking(robot_uid).await;
        {
            let mut latest_status = latest_status_container.lock().unwrap().clone();
            while latest_status != "WAITING_WORKER_TO_PICK" {
                sleep(Duration::from_millis(50)).await;
                latest_status = latest_status_container.lock().unwrap().clone();
            }
        }

        let request_picking_ids: Vec<String> = {
            let mut cloned_picking_ids_container = picking_ids_container.lock().unwrap();
            let received_picking_ids = cloned_picking_ids_container.remove(0);
            assert!(received_picking_ids.len() != 0);
            received_picking_ids
        };

        flody_console.worker_arrived(&request_picking_ids).await;
        {
            let mut latest_status = latest_status_container.lock().unwrap().clone();
            while latest_status != "PICKING" {
                sleep(Duration::from_millis(50)).await;
                latest_status = latest_status_container.lock().unwrap().clone();
            }
        }

        for picking_id in &request_picking_ids {
            let _ = flody_console.get_same_location_pickings(picking_id).await;

            flody_console.complete_picking(picking_id).await;
        }
        desired_robot_status = dlody.consume_desired_topic().await;

        sleep(Duration::from_millis(50)).await;
    }

    /* wait until f/c receives unloading are done */
    dlody.publish_started_unloading(robot_uid).await;
    dlody.publish_unloading(robot_uid).await;

    let mut latest_fc_status = latest_status_container.lock().unwrap().clone();
    while latest_fc_status != "UNLOADING" {
        println!("\n[RUSTLODY] pickings not done, proceeding to sleep\n");

        latest_fc_status = latest_status_container.lock().unwrap().clone();
        sleep(Duration::from_millis(500)).await;
    }

    /* start unloading */
    let total_unloadings = flody_console.get_total_unloadings().await;

    let mut workgroup_ids: Vec<&str> = vec![];
    for unloading in &total_unloadings.in_progresses {
        workgroup_ids.push(&unloading.workgroup_id);
    }
    flody_console.complete_unloading(&workgroup_ids).await;

    dlody.consume_desired_topic().await;

    /* Simulation finished; test states */
    let final_fc_status = latest_status_container.lock().unwrap().clone();
    assert!(final_fc_status == "MOVING_FOR_LOADING".to_string());

    println!("\n\n===END SUCCESS FLOW TEST===\n\n");
}
