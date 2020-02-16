
#![allow(non_snake_case)]
fn main () {

    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    // without type annotation
    let a_tuple = (1, true, 'C');

    //with tuple annotation

    let b_tuple : (u8, char, bool) = (2, 'd', false);


    //destructuring - get all elements from tuple
    let (x, y, z) = a_tuple;
    println!("elements extracted with pattern matching/destructuring {} {} {}", x, y, z);

    //we can access a tuple element directly by using a period (.) 
    let myUnsignedInteger8Bit = a_tuple.0;

    let myBool: bool = b_tuple.2;

    println!("some elements extracted with direct access (.) : {} {}", myUnsignedInteger8Bit, myBool);

    // we can return tuples from functiions

    let (an_integere, a_string) = get_tuple_from_function();
}

/*

*/

fn get_tuple_from_function() -> (u32, String) {

    let an_integer : u32 = 48;

    let a_string = String::from("a_string");

    (an_integer, a_string)
}