
// this is needed if you don't want to see warnings
// about the names of varianles in camelCase
#![allow(non_snake_case)]
fn main () {

    //scalar types

    //integers

    // u8  - unsigned 8 bit integer
    let eightBitUnsignedInteger : u8 = 255;

     // u32 - unsigned integer - 32 bits of space
    let unsignedInteger : u32 = 3;

    //i32 - signed integer - 32 bits of space
    let signedInteger : i32 = 3;

    /*
    If you run in debug:

    thread 'main' panicked at 'attempt to add with overflow', src/types.rs:14:13
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
    
    If you run with --release (ex)
    you will not have any error but your code will not work as expected

    */
    let integerOverflow = eightBitUnsignedInteger + 1;


    println!("Some integers: {} {} {}", eightBitUnsignedInteger, unsignedInteger, signedInteger);

    // Float point - are numbers with decimal points
    // - default is f64

    let defaultIsF64 = 3.0;

    // f32 precision
    let floatingPoint32: f32 = 2.0;

    println!("Some floating points: {} {}", defaultIsF64, floatingPoint32)



}