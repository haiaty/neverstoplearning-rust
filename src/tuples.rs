
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
}