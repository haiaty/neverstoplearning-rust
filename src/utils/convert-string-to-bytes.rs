
 use std::str;

fn main() {

    // convert from string to bytes
    let string = "foo";
    println!("{:?}", string.as_bytes()); // prints [102, 111, 111]
    
    
    //convert from bytes to string
    // some bytes, in a vector
    let sparkle_heart = vec![240, 159, 146, 150];

    // We know these bytes are valid, so just use `unwrap()`.
    let sparkle_heart = str::from_utf8(&sparkle_heart).unwrap();
}
