use std::str::FromStr;
use tokio::time::{sleep, Duration};
use colored::Colorize;

use crate::database::db::*;
use crate::flody_console::console::{FlodyConsole, FlodyConsoleState};
use crate::robot::modes::init_successful_robot;

pub async fn run_success_flow() {
    /* setup */
    let wave_file_path = "./resources/wave-long";
    let robot_uid = "dBK39Eak";

    setup_success_flow(wave_file_path).await;
    init_successful_robot(robot_uid.to_string()).await;

    let flody_console = FlodyConsole::new();
    flody_console.init_websocket().await;


    println!("\n\n===START SUCCESS FLOW TEST===\n\n");

    flody_console.http_client.upload_excel(wave_file_path).await;
    sleep(Duration::from_millis(50)).await;

    flody_console.http_client.command_robot_loading().await;

    loop {
        let cloned_status = flody_console.latest_status_container.lock().unwrap().clone();
        let console_status = FlodyConsoleState::from_str(&cloned_status).expect("[Flody Console] | no matching console status");

        match console_status {
            FlodyConsoleState::MOVING_FOR_LOADING => {
                println!("[FLODY_CONSOLE] | moving for loading");
                sleep(Duration::from_millis(500)).await;
            }

            FlodyConsoleState::LOADING => {
                let tracking_number = "6078917960521"; //must be one of the number in wave_file_path file
                let _ = flody_console
                    .http_client
                    .identify_repesentative_invoice_barcode(tracking_number)
                    .await;

                flody_console.http_client.start_work(robot_uid, tracking_number).await;
            },

            FlodyConsoleState::MOVING_FOR_PICKING => {
                println!("{} | moving for picking", "[FLODY_CONSOLE]".yellow());
                sleep(Duration::from_millis(500)).await;
            }

            FlodyConsoleState::WAITING_WORKER_TO_PICK => {
                let request_picking_ids: Vec<String> = {
                    let mut cloned_picking_ids_container = flody_console.picking_ids_container.lock().unwrap();
                    let received_picking_ids = cloned_picking_ids_container.remove(0);
                    assert!(received_picking_ids.len() != 0);
                    received_picking_ids
                };

                flody_console.http_client.worker_arrived(&request_picking_ids).await;

                for picking_id in &request_picking_ids {
                    let _ = flody_console.http_client.get_same_location_pickings(picking_id).await;

                    flody_console.http_client.complete_picking(picking_id).await;
                }
            }

            FlodyConsoleState::PICKING => {
                println!("{} | picking", "[FLODY_CONSOLE]".yellow());
                sleep(Duration::from_millis(500)).await;
            }

            FlodyConsoleState::MOVING_FOR_UNLOADING => {
                println!("{} | moving for unloading", "[FLODY_CONSOLE]".yellow());
                sleep(Duration::from_millis(500)).await;
            }

            FlodyConsoleState::UNLOADING => {
                let total_unloadings = flody_console.http_client.get_total_unloadings().await;

                let mut workgroup_ids: Vec<&str> = vec![];
                for unloading in &total_unloadings.in_progresses {
                    workgroup_ids.push(&unloading.workgroup_id);
                }
                flody_console.http_client.complete_unloading(&workgroup_ids).await;

                break
            }

        }
        sleep(Duration::from_millis(50)).await;
    }

    /* Simulation finished; test states */
    sleep(Duration::from_millis(1000)).await;  // wait for final flody console message
    //
    let final_fc_status = flody_console.latest_status_container.lock().unwrap().clone();
    assert!(final_fc_status == "MOVING_FOR_LOADING".to_string());

    println!("\n\n===END SUCCESS FLOW TEST===\n\n");
}


pub async fn run_halt_flow() {
    let robot_uid = "dBK39Eak";

    setup_halt_flow().await;

    let flody_console = FlodyConsole::new();
    flody_console.init_websocket().await;

    flody_console.http_client.halt_robot(robot_uid).await;
}
