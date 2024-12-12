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
        let de_body: WsRobotInProgress = serde_json::from_str(msg_body).unwrap();
        Some(de_body)
    }

    fn parse_in_progress_status(&self, text: &str) -> Option<String> {
        match self.parse(text) {
            None=> None,
            Some(ws_robot_in_progress) => {
               Some(ws_robot_in_progress.status)
            }
        }
    }

    fn parse_in_progress_pickings(&self, text: &str) -> Option<Vec<String>> {
        match self.parse(text) {
            None=> None,
            Some(ws_robot_in_progress) => {
                println!("{} | [RECV]: {:?}", "[WS]".red().bold(), ws_robot_in_progress);

                if ws_robot_in_progress.status == "WAITING_WORKER_TO_PICK" {
                    return Some(ws_robot_in_progress 
                        .in_progress_pickings
                        .iter()
                        .map(|picking_cmd| picking_cmd.picking_id.clone())
                        .collect()
                    );
                }
                None
            }
        }
    }
}

#[async_trait]
impl ezsockets::ClientExt for PantosStompClient {
    type Call = ();

    async fn on_text(&mut self, text: String) -> Result<(), Error> {
        match self.parse_in_progress_status(&text) {
            None => (),
            Some(new_status) => {
                let mut locked_status = self.latest_status.lock().unwrap();
                *locked_status = new_status
            }
        }
        match self.parse_in_progress_pickings(&text) {
            None => (),
            Some(new_picking_ids) => {
                let mut locked_picking_ids = self.picking_ids.lock().unwrap();
                locked_picking_ids.push(new_picking_ids);
            }
        }
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
