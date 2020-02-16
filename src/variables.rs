


fn main() {

    //===============
    // INFO
    //==============

    // - If a variable is on stack it gets copied with no problems because
    //   it is cheaper to do 
    // - If a variable is on heap, it is not copied without explicit
    // saying it with method "clone"

    //by default variables are immutable.
    // When a variable is immutable, once a value is bound to a name, you can’t change that value.
    let x = '2';
    println!("the value of x is {}", x);

    //but if you want you can make them mutable
    let mut y = 2;

    y = y + 1;
    println!("the value of y is {}", y);

    //=============
    // shadowing
    //==============
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

    //===========
    // Statements
    //============

    // you can't do 
    let x = (let y = 6); //because it is a statement and it doesn't return the var of assignment as oher languages

    //==========
    // SCOPE
    //============
    //  A scope is the range within a program for which an item is valid

    //  The variable is valid from the point at which it’s declared until the end of the current scopeù

    // when a variable goes out of scope, Rust automatically 
    // calls the drop function and cleans up the heap memory for that variable


    //===========
    // MOVE 
    //===========

    // basically when a variable that uses heap is copied onto another one
    // rust will not make a copy of data on heap but will invalidate the first reference


/*
This will give this error:
error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
*/
    let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);

// If we want to perform a deep clone we can use
// the method clone.
// When you see a call to clone, you know that some arbitrary 
// code is being executed and that code may be expensive
let one = String::from("hi");

let two = one.clone();


//=======
// COPY
//========

// scalar types are copied because they are in the stack

}