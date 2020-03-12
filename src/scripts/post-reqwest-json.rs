
use std::error::Error;
use serde_json;
use serde_json::{Value};
use reqwest::Client;

#[derive(Debug)]
pub struct MyReq { 
    domain: String,
    pub client: Client 
}

impl MyReq {

    // constructor of MyReq - a wrapper to hold
    // and share same client for different http calls
    pub fn new(domain: String) -> MyReq  {
    
        //since we want custom options to the client
        // we use a builder to set options
        let client_builder = reqwest::Client::builder(); 
       
        let client_builder = client_builder.cookie_store(true)
        .danger_accept_invalid_hostnames(true) // accept all hostnames -WARNING: you should know what you are doing
        .danger_accept_invalid_certs(true); //accept all certificates - WARNING: you should know what you are doing

        //create the client that will be used
        // on calls
        let client =client_builder.build().unwrap();

        MyReq {
            domain,
            client
        }
    }
}


pub async fn login(myReq: &MyReq) -> Result<(), Box<Error>>{

    let url = "https://<myurltologinwithputorpost>";

    let response = myReq.client.put(url)
        .json(&serde_json::json!([
            {
                "user_name": "my_username",
                "password": "my_password"
            }
        ]))
        .send()
        .await?;

        // here we take the body as text
        let response_body = response.text().await?;

        println!("{:#?}", response_body);

      // since the body is a json we convert it into json
      let v: Value = serde_json::from_str(response_body.as_str())?; 

      println!("{}", v["type"]); //we are assuming in that response json there is a key called "type"

    
    Ok(())


}


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let domain = String::from("mydomain");

    let my_req = MyReq::new(domain);

    //println!("{:#?}", my_req);

    let result = login(&my_req).await;

    match result {
        Ok(value) => {

            println!("some value");

        },
        Err(e) => {
            panic!("error {}", e);
        }
    }



   Ok(()) // since we are in async function we should return

}
