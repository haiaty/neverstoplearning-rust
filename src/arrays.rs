
#![allow(non_snake_case)]
fn main() {

println!("arrays");


//================
// INFO
//==============

// - every element of an array must have the same type
//  - arrays in Rust have a fixed length
// - An array is a single chunk of memory allocated on the stack.

//=========================
// When arrays are useful?
//==========================
// - when you want your data allocated on the stack rather than the heap
// - when you want to ensure you always have a fixed number of elements

//==================================
// declaration 
//==================================
let my_array = [1, 2, 3];

//==================================
// declaration with type and length
//==================================
// Here, i32 is the type of each element. 
// After the semicolon, the number 3 indicates the array contains 3 elements.
let my_array_with_type_annotation : [i32; 3] = [1, 2, 3];


// initialize array with same value
// The array named a will contain 5 elements that will all be set to the value 3 initially
// This is the same as writing let a = [3, 3, 3, 3, 3];
let array_with_same_value = [3; 5];

//==================================
// Array access
//==================================

let first = my_array[0];
println!("first element of array is {}", first);


// If you try to access an index out of bonds the program
// will compile but will panic at runtime with
/* thread 'main' panicked at 'index out of bounds: the len is 5 but the index is
 10', src/main.rs:5:19
 note: Run with `RUST_BACKTRACE=1` for a backtrace.*/

}