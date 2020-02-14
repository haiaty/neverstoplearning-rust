

fn main() {


    // string literal
    let a_string = "hello";

    // String
    // - is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time

    //========
    // create a String from a string literal using the from function
    //============

    let s = String::from("hello");


    println!("{}", s);

}