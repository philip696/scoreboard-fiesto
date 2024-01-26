use lapin::{options::BasicPublishOptions, BasicProperties, Connection, ConnectionProperties};

pub async fn produce_to_rabbitmq(
    host: String,
    username: String,
    password: String,
    routing_key: String,
    message: String,
) -> Result<(), String> {
    let amqp_uri = format!("amqp://{}:{}@{}:5672", username, password, host); //state.lock().unwrap().rabbitmq_host.to_string(); // Replace with your URI

    // Connect to the AMQP server
    let conn = Connection::connect(&amqp_uri, ConnectionProperties::default())
        .await
        .expect("Failed to connect to AMQP server");

    let channel = conn
        .create_channel()
        .await
        .expect("Failed to create a channel");

    channel
        .basic_publish(
            "amq.topic",  // Exchange
            &routing_key, // Routing key (queue name)
            BasicPublishOptions::default(),
            &message.as_bytes().to_vec(),
            BasicProperties::default(),
        )
        .await
        .map_err(|e| e.to_string())
        .map_err(|e| e.to_string())
        .unwrap();

    Ok(())
}
