


/*
error-chain = "0.12.4"
toml = "0.5.8"

https://docs.rs/toml/0.5.8/toml/

 */

use toml::Value;
use std::fs::File;

fn main() {

    let mut file = File::open("checks.toml").chain_err(|| "could not open tom file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let value = contents.parse::<Value>().unwrap();

}