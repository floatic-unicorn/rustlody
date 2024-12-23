use async_trait::async_trait;
use colored::Colorize;
use ezsockets::ClientConfig;
use ezsockets::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::{thread, time};

use crate::stomp::message::WsRobotInProgress;

pub struct PantosStompClient {
    handle: ezsockets::Client<Self>,
    latest_status: Arc<Mutex<String>>,
    picking_ids: Arc<Mutex<Vec<Vec<String>>>>,
}

impl PantosStompClient {
    pub async fn init(latest_status: Arc<Mutex<String>>, picking_ids: Arc<Mutex<Vec<Vec<String>>>>) {
        tracing_subscriber::fmt::init();

        let config = ClientConfig::new("ws://127.0.0.1:8080/ws");
        let (_, future) = ezsockets::connect(
            |handle| PantosStompClient {
                handle,
                latest_status,
                picking_ids,
            },
            config,
        )
        .await;
        thread::sleep(time::Duration::from_secs(2)); // wait for websocket to connect

        tokio::spawn(async move {
            future.await.unwrap();
        });
    }

    fn parse(&self, text: &str) -> Option<WsRobotInProgress> {
        let mut msgs = text.split("\n");
        let header = msgs.next().unwrap();
        if header != "MESSAGE" {
            return None;
        }

        let msgs = text.split("\n");
        let msg_body = msgs.last().unwrap().trim_matches('\0');
        let ws_robot_in_progress: WsRobotInProgress = serde_json::from_str(msg_body).unwrap();

        println!(
            "{} | [RECV]: status={:?}, in_progress_pickings={:?}",
            "[WS]".red().bold(),
            ws_robot_in_progress.status,
            ws_robot_in_progress.in_progress_pickings
        );
        Some(ws_robot_in_progress)
    }

    fn update_on_message(&self, text: &str) {
        match self.parse(text) {
            None => (),
            Some(ws_robot_in_progress) => {
                let status = ws_robot_in_progress.status;

                if status == "WAITING_WORKER_TO_PICK" {
                    let new_picking_ids: Vec<String> = ws_robot_in_progress 
                        .in_progress_pickings
                        .iter()
                        .map(|picking_cmd| picking_cmd.picking_id.clone())
                        .collect();
                    let mut locked_picking_ids = self.picking_ids.lock().unwrap();
                    locked_picking_ids.push(new_picking_ids);
                }

                let mut locked_status = self.latest_status.lock().unwrap();
                *locked_status = status;
            }
        }
    }
}

#[async_trait]
impl ezsockets::ClientExt for PantosStompClient {
    type Call = ();

    async fn on_text(&mut self, text: String) -> Result<(), Error> {
        self.update_on_message(&text);
        Ok(())
    }

    async fn on_binary(&mut self, _bytes: Vec<u8>) -> Result<(), Error> {
        panic!("[WS] received socket binary")
    }

    async fn on_call(&mut self, _call: Self::Call) -> Result<(), Error> {
        panic!("[WS] received socket on_call")
    }

    async fn on_connect(&mut self) -> Result<(), Error> {
        self.handle
            .text("CONNECT\naccept-version:1.2\n\n\0")
            .unwrap();
        self.handle
            .text("SUBSCRIBE\nid:rustlody\ndestination:/topic/fleet/dBK39Eak?concern=inProgress\n\n\0")
            .unwrap();
        Ok(())
    }
}
