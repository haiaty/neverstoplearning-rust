

fn main () {

    //================
    // INFO
    //==============
    // - Expressions evaluate to a resulting value
    // - Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value

    // example of expressions
    // - a simple math operation, such as 5 + 6
    // - calling a function such as my_func()
    // - Calling a macro is an expression

    // -The block that we use to create new scopes, {}, is an expression. See code below :
  
    let x = 5;

    let y = {
        let x = 3; //shadowing x
        x + 1 //note that there is no semicolon. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value
    };
    /*
    {
        let x = 3;
        x + 1
    } 
    is a block that evaluates to 4
    */

    println!("The value of y is: {}", y); // The value of y is: 4


    // An IF is an expression
    // types should match. If the types are mismatched, we’ll get an error
    let condition = true;
    let number = if condition {
        5 // expression evaluates to an integer
    } else {
        6 // expression evaluates to an integer
    };

    println!("The value of number is: {}", number);
}