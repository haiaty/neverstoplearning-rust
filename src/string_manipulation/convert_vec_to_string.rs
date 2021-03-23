
use std::str;
fn main() {

    let sparkle_heart = vec![240, 159, 146, 150];

    println!("{}", str::from_utf8(&sparkle_heart).unwrap());
}