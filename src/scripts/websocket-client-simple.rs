/*

CARGO TOML dependecies for example to run

[dependencies]
tungstenite="0.10.1"
url = "2.1.0"
env_logger = "0.7.1"
serde_json = "1.0"
*/


use tungstenite::{connect, Message};
use url::Url;

fn main() {


     env_logger::init();

    let (mut socket, response) =
        connect(Url::parse("ws://demos.kaazing.com/echo").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket.write_message(Message::Text(r#"
        {
            "event":"sub",
            "params":{}
        }"#.into()))
        .unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {:#?}", msg.to_text());
    }

}
