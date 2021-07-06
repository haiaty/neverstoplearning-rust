
use std::{process, fs};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {

    let filePath =  "/path/to/file.txt";

    // read the content from the file
    let contents = fs::read_to_string(filePath)
        .expect("Something went wrong reading the file");

    // replace the string
    let new = contents.replace("aaaa", "bbbb");

    // write the new content
    // back to the file
    let mut file = OpenOptions::new().write(true).truncate(true).open(filePath).expect("Could not open the file");

    file.write(new.as_bytes()).expect("could not write new content to file");

}