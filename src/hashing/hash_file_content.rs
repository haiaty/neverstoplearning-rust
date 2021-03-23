/**

sha2 = "0.9.3"
*/

extern crate sha2;

use sha2::{Sha256, Digest};
use std::fs::File;

fn main() {

    let mut f = File::open("foo.txt")?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let mut hasher = Sha256::new();

    hasher.update(buffer);

    let current_hash = format!("{:x}", hasher.finalize());
    println!("hash is: {}", current_hash);

}