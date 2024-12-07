use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::message::Message;
use rdkafka::producer::FutureProducer;

pub struct PantosKafkaClient {
    consumer: StreamConsumer,
    producer: FutureProducer,
}

impl Default for PantosKafkaClient {
    fn default() -> Self {
        Self::new()
    }
}

impl PantosKafkaClient {
    pub fn new() -> Self {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "rustlody")
            .set("bootstrap.servers", "127.0.0.1:9092")
            .set("auto.offset.reset", "earliest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .expect("Consumer creation failed");
        let consumer_topics = ["local.fleet.dBK39Eak.desired.json"];
        consumer
            .subscribe(&consumer_topics)
            .expect("[SETUP] Subscription failed");

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "127.0.0.1:9092")
            .set("message.timeout.ms", "5000")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .expect("Producer creation failed");

        println!("[SETUP] kafka client setup success");
        PantosKafkaClient { consumer, producer }
    }

    pub async fn received_command_move_to_loading_zone(&self) {
        match self.consumer.recv().await {
            Err(_err) => println!(
                "[ROBOT-KAFKA] | [SUB] | [ERR] | received_coommand_move_to_loading_zone report failed: {}",
                _err
            ),
            Ok(_msg) => {
                let payload = match _msg.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("[ROBOT-KAFKA] | [SUB] | [ERR] | Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                println!(
                    "[ROBOT-KAFKA] [SUB] | received_coommand_move_to_loading_zone report received: {}",
                    payload
                );
                self.consumer
                    .commit_message(&_msg, CommitMode::Async)
                    .unwrap();
            }
        }
    }
}
