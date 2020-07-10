/*
redis="0.16.0"
redis-streams="0.1.1"
anyhow = "1.0"
serde_json = "1.0"

*/

use redis_streams::{client_open,StreamCommands, StreamReadOptions, StreamReadReply};
use redis::FromRedisValue;

#[allow(non_snake_case)]
fn main() -> anyhow::Result<(), anyhow::Error> {


    let client = client_open("redis://127.0.0.1/0").unwrap();
    let mut con = client.get_connection().unwrap();

    loop {

            let opts = StreamReadOptions::default()
        .group("mygroupname", "consumername");

        //this option is needed to make it block
        // until receive the next element
        // the params 0 means block till receive
        let opts = opts.block(0);

        // the id '>' means the next item not deleivered yet to no other consumer group
        // see documentation: https://redis.io/topics/streams-intro
        let results: StreamReadReply =
        con.xread_options(&["my_stream"], &[">"], opts)?;


        //println!("{:#?}", results);
    
        for streamKey in results.keys.iter() {
    

            for streamId in streamKey.ids.iter() {

                // this will return a RedisValue type
                // ref: https://docs.rs/redis/0.16.0/redis/enum.Value.html
                let payloadRedisvalue = streamId.map.get("payload").unwrap();
        
                // so we need to convert the RedisValue to a type, therefore
                // we use the method "from_redis_value" on a type that we want to convert from
                // see: https://docs.rs/redis/0.16.0/redis/trait.FromRedisValue.html#tymethod.from_redis_value
                let payloadString : String = String::from_redis_value(payloadRedisvalue).unwrap_or("{}".to_string());
        
                let json_value : serde_json::Value  = serde_json::from_str(&payloadString).expect(&format!("error with json {} ", payloadString));
        
                println!("\n {:#?} \n", json_value["event_description"].as_str().unwrap());
                println!("{:#?} \n", payloadString);
            }
            //println!("{:#?}", streamKey);
        }
    }

     
}
