/*
use stomp_rs::client::{Client, ClientBuilder};
use stomp_rs::protocol::frame::Subscribe;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use std::error::Error;
use stomp_rs::protocol::{Frame, ServerCommand};
use std::future::Future;
use std::sync::Arc;


pub struct PantosStompClient {
    //stomp_client: Client,
    subscriber_client: Arc<Client>,
    sender: Sender<Frame<ServerCommand>>,
    receiver: Receiver<Frame<ServerCommand>>,
}

impl PantosStompClient {

    pub async fn new() -> Self {
        let stomp_client =  Client::connect(ClientBuilder::new("http://localhost:8080")).await.unwrap();
        let subscriber_client = Arc::new(stomp_client);
        let (sender, mut receiver): (Sender<Frame<ServerCommand>>, Receiver<Frame<ServerCommand>>) = channel(16);

        PantosStompClient {
            //stomp_client, 
            subscriber_client,
            sender,
            receiver,
        }
    }

    pub async fn listen_in_progress_destination(mut self, robot_uid: &str) {
        println!("[STOMP] listen in progress destination");

        let cloned_client = Arc::clone(&self.subscriber_client);
        tokio::spawn(async move {
            match self.receiver.recv().await {
              Some(frame) => {
                /* process frame */
                println!("[STOMP] received frame {}", frame.body);

                // Send ack to server
                cloned_client.ack(frame.ack().unwrap()).await;
              }
              None => { }
            }
        });

        let destination = String::from("/topic/fleet/") + robot_uid + "?concern=inProgress";
        self.subscriber_client.subscribe(
            Subscribe::new_with_random_id(destination),
            self.sender
        )
        .await
        .unwrap();

        println!("[STOMP] listen in progress destination done");
    }
}
*/

use async_trait::async_trait;
use ezsockets::ClientConfig;
use ezsockets::CloseCode;
use ezsockets::CloseFrame;
use ezsockets::Error;
use std::io::BufRead;
use url::Url;
use std::{thread, time};

enum Call {
    NewLine(String),
}

struct Client {
    handle: ezsockets::Client<Self>,
}

#[async_trait]
impl ezsockets::ClientExt for Client {
    type Call = Call;

    async fn on_text(&mut self, text: String) -> Result<(), Error> {
        tracing::info!("@@@received message: {text}");
        println!("####received message: {text}");
        println!("####1please");
        Ok(())
    }

    async fn on_binary(&mut self, bytes: Vec<u8>) -> Result<(), Error> {
        tracing::info!("received bytes: {bytes:?}");
        println!("####2received message: {bytes:?}");
        println!("####2please");
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), Error> {
        match call {
            Call::NewLine(line) => {
                println!("####3please");
                if line == "exit" {
                    tracing::info!("exiting...");
                    self.handle
                        .close(Some(CloseFrame {
                            code: CloseCode::Normal,
                            reason: "adios!".to_string(),
                        }))
                        .unwrap();
                    return Ok(());
                }
                tracing::info!("sending {line}");
                self.handle.text(line).unwrap();
            }
        };
        Ok(())
    }
}

pub async fn run() {
    tracing_subscriber::fmt::init();
    let mut args = std::env::args();
    let url = args
        .nth(1)
        .unwrap_or_else(|| "ws://127.0.0.1:8080/ws".to_string());
    let url = Url::parse(&url).unwrap();
    let config = ClientConfig::new(url);
    let (handle, future) = ezsockets::connect(|handle| Client { handle }, config).await;

    tokio::spawn(async move {
        handle.text("SUBSCRIBE\nid:sub-0\ndestination:/topic/fleet/dBK39Eak?concern=inProgress\n\0").unwrap();
    });
    future.await.unwrap();
}
