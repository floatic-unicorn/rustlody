use crate::database::db::setup_halt_flow;
use crate::flody_console::console::FlodyConsole;

pub async fn run_halt_flow() {
    let robot_uid = "dBK39Eak";

    setup_halt_flow().await;

    let flody_console = FlodyConsole::new();
    flody_console.init_websocket().await;

    flody_console.http_client.halt_robot(robot_uid).await;
    // TOOD: assert!(robot has halted)
}
