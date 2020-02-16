

fn main() {

    let mut s = String::from("Hi");

    let len = calculate_lenght(&s);

    println!("the string is: {} and the lenght is : {}", s, len);


    let reference = &mut s;
    //let mut reference2 = &mut s; /*cannot borrow s as mutable more than once a time*/

     println!("reference {}", reference);
     println!("the string is: {} and the lenght is : {}", s, len);

}


fn calculate_lenght(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {

}