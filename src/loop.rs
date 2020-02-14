

fn main () {

    // The loop keyword tells Rust to execute a block of code over and over again forever 
    // or until you explicitly tell it to stop using break


    let mut a_flag_number = 0;

    loop {

        if a_flag_number == 3 {
            println!("stopping!");
            break
        }

        println!("again!");

        a_flag_number += 1;
    }

    /*
    you might need to pass the result of that operation to the rest of your code. 
    To do this, you can add the value you want returned after the break expression you use to stop the loop;
    */

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // we use a semicolon to end the statement that assigns the value to result

    println!("The result is {}", result);





}