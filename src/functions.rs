
fn before_main() {
    println!("before main");
}

fn main() {

//================
// INFO
//==============

// - uses snake case as the conventional style for function
// - you can declare function before main as well
// - In function signatures, you must declare the type of each parameter

//  Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.

//================
// EXAMPLES
//==============
before_main();

my_function_without_params();

my_function_with_param(2);

let result = my_function_with_return();
println!("Value of x is  {}", result); //Value of x is 5

}


fn my_function_without_params() {
    println!("hi");
}

// - param type should be declared
fn my_function_with_param(param : u32) {
    println!("functin with param {}", param);
}

// - the return type should be declared if function return values
// - return values should not be named
fn my_function_with_return() -> i32 {
    5 //note the missing semicolon
}