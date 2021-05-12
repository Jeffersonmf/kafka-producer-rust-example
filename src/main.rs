use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord};
use std::time::Duration;
use chrono::{Local, DateTime};
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("ARGUMENTOS RECEBIDOS{:?}", args);

    if args.len() < 5 {
        return;
    }

    let dt: DateTime<Local> = Local::now();

    println!("Finished pooling at {0}", dt);    

    let producer: BaseProducer = ClientConfig::new()    
    .set("bootstrap.servers", &args[1])
    .create()
    .expect("Producer creation error");

    producer.send(
        BaseRecord::to(&args[2])
            .payload(&args[3])
            .key(&args[4]),
    ).expect("Failed to enqueue");    

    // Poll at regular intervals to process all the asynchronous delivery events.
    for _ in 0..10 {
        producer.poll(Duration::from_millis(100));
    }

    // And/or flush the producer before dropping it.
    producer.flush(Duration::from_secs(1));
}