

fn main () {

    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    // without type annotation
    let a_tuple = (1, true, 'C');

    //with tuple annotation

    let b_tuple : (u8, char, bool) = (2, 'd', false);


    //get all elements from tuple
    let (x, y, z) = a_tuple;
    println!("elements extracted {} {} {}", x, y, z);

    println!("tuples");
}