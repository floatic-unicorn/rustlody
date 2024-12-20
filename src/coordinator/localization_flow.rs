use tokio::time::{sleep, Duration};

use crate::database::db::*;
use crate::flody_console::console::FlodyConsole;
use crate::kafka::pantos_client::PantosKafkaClient;
use crate::robot::dlody::Dlody;


pub async fn run_localization_flow_booting_and_no_job() {
    let robot_uid = String::from("dBK39Eak");
    setup_localization_flow_booting_and_no_job().await;
    
    let dlody = Dlody::new(robot_uid);

    let flody_console = FlodyConsole::new();
    flody_console.init_websocket().await;

    dlody.publish_off_to_on_switch().await;
    sleep(Duration::from_millis(1500)).await;
    assert!(flody_console.get_status() == "BOOTING");

    dlody.publish_location_scan().await;
    sleep(Duration::from_millis(1500)).await;
    assert!(flody_console.get_status() == "IDLE");
}

pub async fn run_localization_flow_booting_and_has_picking_job() {
    let robot_uid = String::from("dBK39Eak");
    setup_localization_flow_booting_and_has_picking_job().await;
    
    let dlody = Dlody::new(robot_uid);

    let flody_console = FlodyConsole::new();
    flody_console.init_websocket().await;

    dlody.publish_off_to_on_switch().await;
    sleep(Duration::from_millis(1500)).await;
    assert!(
        flody_console.get_status() == "BOOTING",
        "status mistmatch: expected `BOOTING`, was `{}`", flody_console.get_status()
    );

    dlody.publish_location_scan().await;
    sleep(Duration::from_millis(1500)).await;
    assert!(
        flody_console.get_status() == "MOVING_FOR_PICKING",
        "status mistmatch: expected `MOVING_FOR_PICKING`, was `{}`", flody_console.get_status()
    );
}

pub async fn run_localization_flow_booting_and_was_emergency_stopped() {
    let robot_uid = String::from("dBK39Eak");
    setup_localization_flow_booting_and_was_emergency_stopped().await;
    
    let dlody = Dlody::new(robot_uid);

    let flody_console = FlodyConsole::new();
    flody_console.init_websocket().await;

    dlody.publish_off_to_on_switch().await;
    sleep(Duration::from_millis(1500)).await;
    assert!(
        flody_console.get_status() == "BOOTING",
        "status mistmatch: expected `BOOTING`, was `{}`", flody_console.get_status()
    );

    dlody.publish_error_status().await;
    sleep(Duration::from_millis(1500)).await;
    assert!(
        flody_console.get_status() == "EMERGENCY_STOPPED",
        "status mistmatch: expected `EMERGENCY_STOPPED`, was `{}`", flody_console.get_status()
    );
}
