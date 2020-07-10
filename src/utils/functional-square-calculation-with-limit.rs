

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn main () {

    // we want to sum all the squares of a sequence of numbers till a limit

    let limit = 500;

    let mut sum = 0;


    //============
    // PROCEDURAL
    //=============

    for i in 0.. //0.. means from 0 to infinity
    {
        let square = i * i;

        if square > limit  { break; } // we have reached the limit
        else if is_even(square) { sum += square;}
    }

    println!("loop sum procedural way = {}", sum);

    //============
    // FUNCTIONAL
    //=============

    let sum2 = (0..)//means a sequence from 0 to infinity
    .map(|x| x * x) //transform each element to its square
    .take_while(|&x| x < limit) //takes elements till a condition is true, in this case the sqauer less than limit
    .filter(|x| is_even(*x)) // filter the even elements
    .fold(0, |sum, x| sum +x ); // fold reduce the sequence to a single value


    println!("loop sum functional way = {}", sum2);
}