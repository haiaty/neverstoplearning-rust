
fn say_hello() {
    println!("Hello");
}

fn main () {

    //you can store functions in a variable
    let sh = say_hello;

    //...and then call it
    sh();

    // function defined inline that captures the scope

    // explanation line of code

    // |x:i32|  -- this is the parameters of the close with their type
    // -> i32   -- this is the return type of the clsodure
    // { x + 1 } -- this is the body of closure
    let plus_one = |x:i32| -> i32 { x + 1 };

    let a = 6;

    println!("{} + 1 =  {}", a, plus_one(a));
    

    // we can also not specify param types and return types and let the compiler
    // guess it for us

    let two = 2;

    let plus_two = |x| 
    { 
        x + two; //note that the two variable is automatically borrowed as immutable in the closure
    };


    //
    let mut three = 3; //simply a mutable variable

    // now if we want to use
    {

    }


}