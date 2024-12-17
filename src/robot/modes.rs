use std::str::FromStr;
use tokio::time::{sleep, Duration};

use super::dlody::{Dlody, DlodyCommand};
use crate::kafka::pantos_client::PantosKafkaClient;

pub async fn init_successful_robot(robot_uid: String) {
    let dlody = Dlody::new(String::clone(&robot_uid));
    
    tokio::spawn(async move {
        let mut is_running = true;
        let mut is_wave_done = false;

        while is_running {
            let consumed_state = dlody.consume_desired_topic().await;
            let state = DlodyCommand::from_str(&consumed_state).expect("[ROBOT] | no matching dlody state");

            match state {
                DlodyCommand::LOADING => {
                    if is_wave_done == true {
                        is_running = false;
                        continue
                    }

                    dlody.publish_started_loading(&robot_uid).await;
                    sleep(Duration::from_millis(100)).await;
                    dlody.publish_loading(&robot_uid).await;
                },

                DlodyCommand::PICKING => {
                    dlody.publish_picking(&robot_uid).await;
                },

                DlodyCommand::UNLOADING => {
                    dlody.publish_started_unloading(&robot_uid).await;
                    dlody.publish_unloading(&robot_uid).await;

                    is_wave_done = true;
                },

                DlodyCommand::TRAVELING => panic!("[ROBOT] | not used topic"),
                DlodyCommand::WAITING => panic!("[ROBOT] | not used topic"),
                DlodyCommand::WAITING_FOR_UNLOADING => panic!("[ROBOT] | not used topic"),
                DlodyCommand::UNPAUSED => panic!("[ROBOT] | not used topic"),
            }
        }

        println!("[ROBOT] | spawned robot finished task");
    });
}
