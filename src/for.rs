
fn main () {


    //The safety and conciseness of for loops make them the most commonly used loop construct in Rust. 

    let an_array = [1, 4, 7, 10];

    for element in an_array.iter() {
        println!("the value is: {}", element);
    }
}