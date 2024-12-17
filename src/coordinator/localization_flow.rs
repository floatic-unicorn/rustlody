use tokio::time::{sleep, Duration};

use crate::database::db::setup_localization_flow_booting_and_no_job;
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
