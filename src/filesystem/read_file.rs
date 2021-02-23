
use std::fs;

fn main() {
    let entry = "/path/of/file";

    let contents = fs::read_to_string(entry)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}