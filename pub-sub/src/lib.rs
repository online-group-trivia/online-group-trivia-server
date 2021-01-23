use redis;

pub fn subscribe_to_topic(topic_name: &str) {
    // TODO: connection pool, Use fred crate maybe
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    let mut pubsub = con.as_pubsub();
    pubsub.subscribe(topic_name).unwrap();

    // TODO: Event loop
    loop {
        let msg = pubsub.get_message().unwrap();
        let payload : String = msg.get_payload().unwrap();
        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }
}

