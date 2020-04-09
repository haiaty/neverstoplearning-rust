/*

[dependencies]
lapin = "0.35.0"
env_logger="0.7.0"
futures-executor="0.3.0"
futures-util="0.3.0"
tcp-stream="0.1.0"
log = "0.4.0"
tokio = { version = "0.2", features = ["full"] }

*/


use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties, Result,
};



#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
   
        let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;

        println!("CONNECTED");

        let channel_a = conn.create_channel().await?;

        let queue = channel_a
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

            println!("Declared queue {:?}", queue);

     
        let payload = b"Hello world!";

        
        channel_a
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                payload.to_vec(),
                BasicProperties::default(),
            )
            .await?;
        
            println!("message published");

        Ok(())
}
