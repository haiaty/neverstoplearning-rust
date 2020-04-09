
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

use futures_util::{future::FutureExt, stream::StreamExt};
use lapin::{
    options::*, types::FieldTable, Connection, ConnectionProperties, Result,
};

use std::str;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
  
        let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;


        println!("CONNECTED");

        let channel_a = conn.create_channel().await?;
        let channel_b = conn.create_channel().await?;

        let queue = channel_a
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

            println!("Declared queue {:?}", queue);

        let consumer = channel_b
            .clone()
            .basic_consume(
                "hello",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

    
            println!("will consume");
            consumer
                .for_each(move |delivery| {

                    println!("raw message: {:?}", delivery);

                    let delivery = delivery.expect("error caught in in consumer");

                    let data : Vec<u8> = delivery.data;

                    //if you want the text of the message
                    let text = str::from_utf8(&data).unwrap();

                    println!("received message: {:?}", text);

                    channel_b
                        .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                        .map(|_| ())
                })
                .await;
        

        Ok(())

 
}
