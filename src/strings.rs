// resources to read about
// - https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html
// - https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str/24159933#24159933

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
    
    //=========
    // remove the last char of a string
    //==============
    let mut s = String::from("Hello");
    
    s.truncate(s.len()-1)

}
