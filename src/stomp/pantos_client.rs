use crate::stomp::message::WsRobotInProgress;
use async_trait::async_trait;
use ezsockets::ClientConfig;
use ezsockets::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::{thread, time};

pub struct PantosStompClient {
    handle: ezsockets::Client<Self>,
    picking_ids: Arc<Mutex<Vec<String>>>,
}

impl PantosStompClient {
    pub async fn init(picking_ids: Arc<Mutex<Vec<String>>>) {
        tracing_subscriber::fmt::init();

        let config = ClientConfig::new("ws://127.0.0.1:8080/ws");
        let (_, future) = ezsockets::connect(
            |handle| PantosStompClient {
                handle,
                picking_ids,
            },
            config,
        )
        .await;
        thread::sleep(time::Duration::from_secs(3)); // wait for websocket to connect

        tokio::spawn(async move {
            future.await.unwrap();
        });
    }

    pub fn parse(&self, text: String) -> Option<Vec<String>> {
        let mut msgs = text.split("\n");
        let header = msgs.next().unwrap();
        if header != "MESSAGE" {
            return None;
        }

        let msg_body = msgs.last().unwrap().trim_matches('\0');
        let de_body: WsRobotInProgress = serde_json::from_str(msg_body).unwrap();
        tracing::info!("[WS] | [RECV]: deserialized = {:?}", de_body);

        Some(
            de_body
                .in_progress_pickings
                .iter()
                .map(|picking_cmd| picking_cmd.picking_id.clone())
                .collect(),
        )
    }
}

#[async_trait]
impl ezsockets::ClientExt for PantosStompClient {
    type Call = ();

    async fn on_text(&mut self, text: String) -> Result<(), Error> {
        match self.parse(text) {
            None => (),
            Some(new_picking_ids) => {
                let mut locked_picking_ids = self.picking_ids.lock().unwrap();
                locked_picking_ids.truncate(0);
                locked_picking_ids.extend(new_picking_ids);
            }
        }
        Ok(())
    }

    async fn on_binary(&mut self, bytes: Vec<u8>) -> Result<(), Error> {
        panic!("[WS] received socket binary")
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), Error> {
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
