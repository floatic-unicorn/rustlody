use std::time::Duration;
use std::{thread, time};

use colored::Colorize;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::message::Message;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use strum_macros::EnumString;

use crate::kafka::conn::{make_consumer, make_producer};
use crate::kafka::desired::DesiredMessage;
use crate::kafka::pantos_client::PantosKafkaClient;
use crate::kafka::report::ReportMessage;

#[allow(non_camel_case_types)]
#[derive(EnumString)]
pub enum DlodyState {
    EMERGENCY_STOPPED(String),
    RECOVERED_FROM_EMERGENCY_STOP(String),
    FAILED_TO_UNPAUSE(String),
    STARTED_PICKING(String),
    PICKING(String),
    STARTED_WAITING_FOR_UNLOADING(String),
    WAITING_FOR_UNLOADING(String),
    STARTED_WAITING(String),
    WAITING(String),
    STARTED_UNLOADING(String),
    UNLOADING(String),
    STARTED_TRAVELING(String),
    ARRIVED_AT_POINT(String)
}

pub struct Dlody {
    consumer: StreamConsumer,
    producer: BaseProducer,
}

impl Default for Dlody {
    fn default() -> Self {
        Self::new()
    }
}

impl Dlody {
    pub fn new() -> Self {
        let consumer = make_consumer();
        let producer = make_producer();
        println!("[SETUP] kafka client setup success");
        Dlody { consumer, producer }
    }
}

impl PantosKafkaClient for Dlody {
    async fn consume_desired_topic(&self) -> String {
        thread::sleep(time::Duration::from_secs(1));

        match self.consumer.recv().await {
            Err(_err) => panic!(
                "[ROBOT-KAFKA] | [SUB] | [ERR] | consume desired topic failed: {}",
                _err
            ),
            Ok(_msg) => {
                let payload = match _msg.payload_view::<str>() {
                    None => "",
                    Some(Ok(m)) => m,
                    Some(Err(e)) => panic!(
                        "[ROBOT-KAFKA] | [SUB] | [ERR] | consume desired topic deserialization error: {:?}",
                        e
                    )
                };
                self.consumer
                    .commit_message(&_msg, CommitMode::Async)
                    .unwrap();

                let deserialized: DesiredMessage = serde_json::from_str(&payload).unwrap();
                println!(
                    "{} | [SUB] | consumed robot status={}, message={}",
                    "[ROBOT]".blue().bold(),
                    deserialized.payload.state,
                    payload
                );
                return deserialized.payload.state;
            }
        }
    }

    fn publish_reported_message(&self, robot_uid: &str, message: ReportMessage) {
        let topic = format!("local.fleet.{robot_uid}.reported.json");
        let serialized_msg = serde_json::to_string(&message).unwrap();

        self.producer
            .send(
                BaseRecord::to(&topic)
                    .payload(&serialized_msg)
                    .key(robot_uid),
            )
            .expect("[ROBOT-KAFKA] | [PUB] failed to enqueue");
        self.producer.poll(Duration::from_millis(100));
        self.producer.flush(Duration::from_secs(1));
        println!(
            "{} | [PUB] published to {}: {}",
            "[ROBOT]".green().bold(),
            topic,
            serialized_msg
        );
    }

    async fn publish_started_loading(&self, robot_uid: &str) {
        let message = ReportMessage::new("STARTED_TRAVELING", 0.0, robot_uid);
        self.publish_reported_message(robot_uid, message);
    }

    async fn publish_loading(&self, robot_uid: &str) {
        let message = ReportMessage::new("ARRIVED_AT_POINT", 0.0, robot_uid);
        self.publish_reported_message(robot_uid, message);
    }

    async fn publish_started_picking(&self, robot_uid: &str) {
        let message = ReportMessage::new("STARTED_PICKING", 0.0, robot_uid);
        self.publish_reported_message(robot_uid, message);
    }

    // Deprecated?
    //async fn publish_waiting_worker_to_pick(&self, robot_uid: &str) {}

    async fn publish_picking(&self, robot_uid: &str) {
        let message = ReportMessage::new("PICKING", 0.0, robot_uid);
        self.publish_reported_message(robot_uid, message);
    }

    async fn publish_started_unloading(&self, robot_uid: &str) {
        let message = ReportMessage::new("STARTED_UNLOADING", 0.0, robot_uid);
        self.publish_reported_message(robot_uid, message);
    }

    async fn publish_unloading(&self, robot_uid: &str) {
        let message = ReportMessage::new("UNLOADING", 0.0, robot_uid);
        self.publish_reported_message(robot_uid, message);
    }

    async fn publish_arrived_at_emergency_position(&self, _: &str) {}

    async fn publish_arrived_at_recovered_position(&self, _: &str) {}
}
