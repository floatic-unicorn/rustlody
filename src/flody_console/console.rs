use reqwest::Client;
use std::sync::{Arc, Mutex};
use strum_macros::EnumString;

use crate::http::client::PantosHttpClient;
use crate::stomp::pantos_client::PantosStompClient;

#[allow(non_camel_case_types)]
#[derive(EnumString)]
pub enum FlodyConsoleState {
    MOVING_FOR_LOADING,
    LOADING,
    MOVING_FOR_PICKING,
    WAITING_WORKER_TO_PICK,
    PICKING,
    MOVING_FOR_UNLOADING,
    UNLOADING,
}

pub struct FlodyConsole {
    pub http_client: PantosHttpClient,
    pub latest_status_container: Arc<Mutex<String>>,
    pub picking_ids_container: Arc<Mutex<Vec<Vec<String>>>>,
}

impl Default for FlodyConsole {
    fn default() -> Self {
        Self::new()
    }
}

impl FlodyConsole {
    pub fn new(
    ) -> Self {
        let http_client =  PantosHttpClient {
            url: String::from("http://localhost:8080"),
            client: Client::new(),
        };
        let picking_ids_container = Arc::new(Mutex::new(vec![]));
        let latest_status_container = Arc::new(Mutex::new("".to_string()));

        FlodyConsole { 
            http_client ,
            latest_status_container,
            picking_ids_container,
        }
    }

    pub async fn init_websocket(&self) {
        PantosStompClient::init(
            self.latest_status_container.clone(),
            self.picking_ids_container.clone(),
        )
        .await;
    }

    pub fn get_status(&self) -> String {
        return self.latest_status_container.lock().unwrap().clone();
    }

    pub async fn get_in_progress_picking_ids(&self) -> Vec<String> {
        let request_picking_ids: Vec<String> = {
            let mut cloned_picking_ids_container = self.picking_ids_container.lock().unwrap();
            let received_picking_ids = cloned_picking_ids_container.remove(0);
            assert!(received_picking_ids.len() != 0);
            received_picking_ids
        };
        request_picking_ids
   }
}
