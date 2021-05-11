use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord};
use std::time::Duration;
use chrono::{Local, DateTime};

fn main() {

    let dt: DateTime<Local> = Local::now();

    println!("Finished pooling at {0}", dt);    

    let producer: BaseProducer = ClientConfig::new()    
    .set("bootstrap.servers", "localhost:9092")
    .create()
    .expect("Producer creation error");

    producer.send(
        BaseRecord::to("teste")
            .payload("this is the payload")
            .key("and this is a key"),
    ).expect("Failed to enqueue");    

    // Poll at regular intervals to process all the asynchronous delivery events.
    for _ in 0..10 {
        producer.poll(Duration::from_millis(100));
    }

    // And/or flush the producer before dropping it.
    producer.flush(Duration::from_secs(1));
}