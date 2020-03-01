

fn main () {

    //  it encodes the very common scenario in which 
    // a value could be something or it could be nothing.

    //this functionality can prevent bugs that are 
    //extremely common in other programming languages.

    //why is having Option<T> any better than having null?
    //In short, because Option<T> and T (where T can be any type) 
    //are different types, the compiler won’t let us use an Option<T> value as if it were definitely a valid value. 

    //Generally, this helps catch one of the most common issues with null: 
    //assuming that something isn’t null when it actually is.

    //In order to have a value that can possibly be null, 
    //you must explicitly opt in by making the type of that value Option<T>.
    //Everywhere that a value has a type that isn’t an Option<T>, 
    //you can safely assume that the value isn’t null


    // some examples of using Option values to hold number types and string types:

    let some_number = Some(5);

    let some_string = Some("a literal string");

    //If we use None rather than Some, we need to tell Rust what type of Option<T>
    // we have, because the compiler can’t infer the type that the Some
    // variant will hold by looking only at a None value.
    let absent_number: Option<u32> = None;

    //So, how do you get the T value out of a Some variant when you have 
    //a value of type Option<T> so you can use that value?
    //In general, in order to use an Option<T> value, you want to have code that will handle each variant.
}