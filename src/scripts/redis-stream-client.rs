/*
redis="0.16.0"
redis-streams="0.1.1"
*/

use redis_streams::{client_open,Connection,StreamCommands};
use redis::AsyncCommands;

fn main()  {

    let client = client_open("redis://127.0.0.1/0").unwrap();
    let mut con = client.get_connection().unwrap();

    let result = StreamCommands::xinfo_stream(&mut con, "stream-name")?;
    
    // other examples: StreamCommands::xack(&mut con, "stream-name")?;
     
     panic!("{:#?}", result);
   
}
