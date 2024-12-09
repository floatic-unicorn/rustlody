use async_trait::async_trait;
use ezsockets::ClientConfig;
use ezsockets::CloseCode;
use ezsockets::CloseFrame;
use ezsockets::Error;
use std::io::BufRead;
use std::{thread, time};
use url::Url;

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
        Ok(())
    }

    async fn on_binary(&mut self, bytes: Vec<u8>) -> Result<(), Error> {
        panic!("[WS] received unwatned binary")
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), Error> {
        tracing::info!("### on call !!!");
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
    let config = ClientConfig::new("ws://127.0.0.1:8080/ws");
    let (handle, future) = ezsockets::connect(|handle| Client { handle }, config).await;
    thread::sleep(time::Duration::from_secs(3)); // wait for websocket to connect

    tokio::spawn(async move {
        handle.text("CONNECT\naccept-version:1.2\n\n\0").unwrap();
        handle
            .text("SUBSCRIBE\nid:sub-0\ndestination:/topic/fleet/dBK39Eak?concern=inProgress\n\n\0")
            .unwrap();

        //thread::sleep(time::Duration::from_secs(3));

        future.await.unwrap();
        tracing::info!("*** end ***");
    });
}
