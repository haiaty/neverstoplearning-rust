


fn main() {

    //by default variables are immutable.
    // When a variable is immutable, once a value is bound to a name, you can’t change that value.
    let x = '2';
    println!("the value of x is {}", x);

    //but if you want you can make them mutable
    let mut y = 2;

    y = y + 1;
    println!("the value of y is {}", y);

    //shadowing
    /*
    you can declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable. 
    Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what appears when the variable is used.
    */

    let be = 3;

    //it shadows x by repeating let x =, 
    //taking the original value and adding 1 so the value of x is then 6
    //By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    let be = be + 2;

    println!("the value of be is {be}", be=be);

    /*
    The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again,
     we can change the type of the value but reuse the same name
    */
    let spaces = "    ";

    // here we are shadowing and changing the type
    let spaces = spaces.len();

    println!("Lenght of spaces is {}", spaces);





    // contants are always immutable. you can't change with mut
    /*
    Constants can be declared in any scope, including the global scope, 
    which makes them useful for values that many parts of code need to know about.

    Rust’s naming convention for constants is to use all uppercase with underscores between words,
     and underscores can be inserted in numeric literals to improve readability

     Constants are valid for the entire time a program runs, within the scope they were declared in, making them a useful choice for values in 
     your application domain that multiple parts of the program might need to know about
    */
    
    //NOTE: the type of the value must be annotated
    const MAX_VALUE: u32 = 100_000; //underscores can be inserted in numeric literals to improve readability
    println!("the value of the costante is:  {}", MAX_VALUE)

}