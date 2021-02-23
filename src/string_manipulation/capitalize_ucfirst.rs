
/*

[dependencies]
voca_rs = "0.3.0"

*/
use voca_rs::*;

fn main() {


    let string_capitalized = case::capitalize(String::from("tocapitalize"), &true); //&true indicates to keep the rest of string lowercase

    println!("{:?}", string_capitalized); // prints Tocapitalize

}