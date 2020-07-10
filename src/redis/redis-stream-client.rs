/*
redis="0.16.0"
redis-streams="0.1.1"
anyhow = "1.0"
serde_json = "1.0"
*/

use redis_streams::{client_open,Connection,StreamCommands};
use redis::FromRedisValue;


fn main() -> anyhow::Result<(), anyhow::Error> {

    // NOTE: this connection is not async I/O because
    // till now there is no support for async I/O for redis streams
    let client = client_open("redis://127.0.0.1/0").unwrap();
    let mut con = client.get_connection().unwrap();

    // take information
    // about the stream
    let result = StreamCommands::xinfo_stream(&mut con, "stream-name")?
      println!("{:#?}", result);
    
    // take a given number of elements 
    // from a stream
    let result = StreamCommands::xrange_count(&mut con, "stream-name", "-", "+", 5)?;

    for streamId in result.ids.iter() {

        // we are assuming that we have a key called "payload"
        // this will return a RedisValue type
        // ref: https://docs.rs/redis/0.16.0/redis/enum.Value.html
        let payloadRedisvalue = streamId.map.get("payload").unwrap();

        // so we need to convert the RedisValue to a type, therefore
        // we use the method "from_redis_value" on a type that we want to convert from
        // see: https://docs.rs/redis/0.16.0/redis/trait.FromRedisValue.html#tymethod.from_redis_value
        let payloadString : String = String::from_redis_value(payloadRedisvalue).unwrap_or("{}".to_string());

        // then we convert the json string into a json Value in order
        // to work with it
        let json_value : serde_json::Value  = serde_json::from_str(&payloadString).expect(&format!("error with json {} ", payloadString));

        println!("{:#?}", json_value["event_description"]);
    }
   
}
