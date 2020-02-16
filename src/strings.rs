

fn main() {

    //===============
    // string literal
    //================
    // - string literals are immutable; the type of a_string here is &str  and it is an immutable reference.
    let a_string = "hello";

    // String
    // - is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time

    //========
    // create a String from a string literal using the from function
    //============

    let s = String::from("hello");


    println!("{}", s);

}