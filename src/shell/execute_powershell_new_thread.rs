/*

1. execute the command 'daml damlc inspect-dar --json <path>' from powershell

2. take results and create a json
  
3. take the value of the property 'main_package_id'


*/

use dotenv::dotenv;
use std::env;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::string::String;

pub fn run() -> String {


    // Load environment variables from .env file
    dotenv().ok();

    let file_path =  env::var("SUBSCRIPTION_DAR_PATH").unwrap();


    // Spawn a new thread to execute the PowerShell command
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    thread::spawn(move || {
        let output = std::process::Command::new("powershell")
            .arg("-Command")
            .arg(format!("daml damlc inspect-dar --json {}", file_path))
            .output()
            .expect("failed to execute process");
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        tx.send(result).unwrap();
    });

    // Wait for the PowerShell command to finish and print its output
    let result = rx.recv().unwrap();
    //println!("{}", result);

    let data: serde_json::Value = serde_json::from_str(&result).unwrap();
    let main_package_id = data["main_package_id"].as_str().unwrap();

    return String::from(main_package_id);
}
