//resources:
// https://doc.rust-lang.org/std/option/enum.Option.html
// https://blog.burntsushi.net/rust-error-handling/#the-option-type

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

    //================
    // METHODS
    //================

    // .is_some()  
    //-----------
    // is used to check if we have some value
    
    let some_number: Option<u32> = Some(6);

    assert_eq!(some_number.is_some(), true);

    // .as_ref() 
    //-----------
    // take an Option to a reference to the value inside the original.
    // think of it like: "I'm borrowing the value of an Option into another Option"

    let some_number: Option<u32> = Some(6);

    let a = some_number.as_ref();

    println!("{:#?}", a);


    // .as_mut()
    //-----------
    // If you want to modify the inner value of an option. 
    // so you have a mutable reference so you can modify the innr value
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {},
    }
    assert_eq!(x, Some(42));

    // .expect()
    //--------------
    // get the value of an Option or panic with a message
    let x = Some("Pippo");

    println!("{:?}", x.expect("Custom message if panic"));

     // .unwrap_or()
    //--------------
    // get the value of an Option but if there is None it return a the value passes
    let x = None;

    println!("{:?}", x.unwrap_or("some in case of none"));

    // .unwrap_or_else()
    //--------------
    // get the value of an Option or execute the closure and take the result of
    // its execution

    let o = None;

    println!("{:?}", o.unwrap_or_else(|| 2 * 2))

}