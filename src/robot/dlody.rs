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
use crate::kafka::status::StatusMessage;

#[allow(non_camel_case_types)]
#[derive(EnumString)]
pub enum DlodyState {
    EMERGENCY_STOPPED,
    RECOVERED_FROM_EMERGENCY_STOP,
    FAILED_TO_UNPAUSE,
    STARTED_PICKING,
    PICKING,
    STARTED_WAITING_FOR_UNLOADING,
    WAITING_FOR_UNLOADING,
    STARTED_WAITING,
    WAITING,
    STARTED_UNLOADING,
    UNLOADING,
    STARTED_TRAVELING,
    ARRIVED_AT_POINT,
}

#[allow(non_camel_case_types)]
#[derive(EnumString)]
pub enum DlodyCommand {
    PICKING,
    UNLOADING,
    LOADING,
    WAITING,
    WAITING_FOR_UNLOADING,
    TRAVELING,
    UNPAUSED,
}

pub struct Dlody {
    robot_uid: String,
    consumer: StreamConsumer,
    producer: BaseProducer,
}

impl Default for Dlody {
    fn default() -> Self {
        panic!("[SETUP] kafka client setup failed; default disallowed");
    }
}

impl Dlody {
    pub fn new(robot_uid: String,) -> Self {
        let consumer = make_consumer();
        let producer = make_producer();
        println!("[SETUP] kafka client setup success");
        Dlody { robot_uid, consumer, producer }
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
                    Some(Ok(m)) => m,
                    None => panic!(
                        "[ROBOT-KAFKA] | [SUB] | [ERR] | consume desired topic deserialization error",
                    ),
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
                    "{} | consumed robot status={}, message={}",
                    "[ROBOT] | [SUB]".blue().bold(),
                    deserialized.payload.state,
                    payload
                );
                deserialized.payload.state
            }
        }
    }

    fn publish_reported_message(&self, robot_uid: &str, message: ReportMessage) {
        let topic = format!("local.fleet.{robot_uid}.reported");
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
            "{} | published to {}: {}",
            "[ROBOT] | [PUB]".green().bold(),
            topic,
            serialized_msg
        );
    }

    // TODO: merge with publish_reported_message && change topic name
    fn publish_status_message(&self, message: StatusMessage) {
        let robot_id = String::clone(&self.robot_uid);
        let topic = format!("local.fleet.{robot_id}.status");
        let serialized_msg = serde_json::to_string(&message).unwrap();

        self.producer
            .send(
                BaseRecord::to(&topic)
                    .payload(&serialized_msg)
                    .key(&robot_id),
            )
            .expect("[ROBOT-KAFKA] | [PUB] failed to enqueue");
        self.producer.poll(Duration::from_millis(100));
        self.producer.flush(Duration::from_secs(1));
        println!(
            "{} | published to {}: {}",
            "[ROBOT] | [PUB]".green().bold(),
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

    async fn publish_emergency_stop(&self) -> () {
        let robot_id = String::clone(&self.robot_uid);
        let message = StatusMessage::new(robot_id, false, true, true);
        self.publish_status_message(message);
    }

    async fn publish_arrived_at_emergency_position(&self, _: &str) {}

    async fn publish_arrived_at_recovered_position(&self, _: &str) {}

    async fn publish_off_to_on_switch(&self) {
        let robot_id = String::clone(&self.robot_uid);
        let message = StatusMessage::new(robot_id, false, false, false);
        self.publish_status_message(message);
    }

    async fn publish_location_scan(&self) {
        let robot_id = String::clone(&self.robot_uid);
        let message = StatusMessage::new(robot_id, true, false, false);
        self.publish_status_message(message);
    }
}
