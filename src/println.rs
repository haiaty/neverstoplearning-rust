
//this is a comment

fn main() {
    println!("hello");
    println!("I'm a rustacean");

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // Add a println! macro that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. 
    //For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi.
    // (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)
    println!("Pi is roughly {pi}", pi=3.141592)
}