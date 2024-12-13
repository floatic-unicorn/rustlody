use std::str::FromStr;

use super::dlody::{Dlody, DlodyState};
use crate::kafka::pantos_client::PantosKafkaClient;

pub async fn spawn_successful_robot() {
    let dlody = Dlody::new();
    
    tokio::spawn(async move {
        let mut is_running = true;

        while is_running {
            let consumed_state = dlody.consume_desired_topic().await;
            let state = DlodyState::from_str(&consumed_state).unwrap();

            match state {
                DlodyState::STARTED_TRAVELING => {},
                DlodyState::ARRIVED_AT_POINT => {},
                DlodyState::PICKING => {},
                DlodyState::WAITING_FOR_UNLOADING => {},
                DlodyState::UNLOADING => {},
            }
        }
    });
}
