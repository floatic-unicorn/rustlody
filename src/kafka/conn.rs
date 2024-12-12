use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::producer::BaseProducer;


pub fn make_consumer() -> StreamConsumer {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "rustlody")
        .set("bootstrap.servers", "127.0.0.1:9092")
        //.set("auto.offset.reset", "earliest")
        .set("auto.offset.reset", "latest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    let consumer_topics = ["local.fleet.dBK39Eak.desired.json"];
    consumer
        .subscribe(&consumer_topics)
        .expect("[SETUP] Subscription failed");
    return consumer;
}

pub fn make_producer() -> BaseProducer {
    return ClientConfig::new()
        .set("bootstrap.servers", "127.0.0.1:9092")
        .set("client.id", "client-rustlody")
        .set("group.id", "group-rustlody")
        .set("message.timeout.ms", "5000")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Producer creation failed");
}
