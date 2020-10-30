
/**
 * 
surf="2.0.0"
anyhow = "1.0"
serde_json = "1.0"
tokio = { version = "0.2", features = ["full"] }
base64="0.13.0"

 */
use anyhow::{anyhow, Context, Result};
use serde_json::{Value};
use base64::{encode, decode};
use std::str;

#[tokio::main]
async fn main() -> Result<()> {

    //
    // get the state
    //
    // - perform request using surf
    let mut res = surf::get("http://127.0.0.1:8008/state/bdc21042647f8e8458a0f2a2dde2bddd4d069dfb6f26455a2d65fb6f3697c1d705e816")
    .await
    .map_err(|err| anyhow!(err)) // we map the Surf::Error to a anyhow::Error because we are returning an anyhow::Result on main functin
    .context("Failed to fetch state")?; //context is a anyhow method that gives more context to an error

    // - extract the body
    let a = res.body_string()
        .await
        .map_err(|err| anyhow!(err)) 
        .context("Failed to parse the body")?;

      // convert it to a json Value of serde 
      let v: Value = serde_json::from_str(a.as_str())?; 

      // take the payload base64 encoded string
     let payloadBase64Encoded : &str =  v["data"].as_str().unwrap();

     println!("{:#?}", v["data"]);

     // decode it 
     let a = &decode(payloadBase64Encoded).context("Failed to decode the payload base64")?[..];
     let payloadJsonString = str::from_utf8(&a).unwrap();

    println!("{:#?}",  payloadJsonString);


    // convert the json string to json Value
    let state: Value = serde_json::from_str(payloadJsonString)?; 

    println!("{:#?}",  state["key"].as_str().unwrap());


    Ok(()) // since we are in async function we should return
}
