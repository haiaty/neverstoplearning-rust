

fn main () {

    let s = String::from("ok");

    // These ampersands are references, and they allow you to refer 
    // to some value without taking ownership of it

    // The &s1 syntax lets us create a reference that refers to the value of s
    // but does not own it. Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.
    a_borrowing_function(&s);

    println!("also here we have the string {}", s);

    // Note that if we had done:  a_function(s)
    // the a_function would have taken the ownwrship
    // and we would not have access to s variable here


    //..then
    // So what happens if we try to modify something we’re borrowing? 
    // Try the code below Spoiler alert: it doesn’t work!
    change(&s);

    //..but if we do a mutable reference,
    // then also the reference can be changed

    let mut s2 = String::from("a string");

    change_with_mut(&mut s2);

    println!("the value of s2 has changed, now it is {}", s2);  //the value of s2 has changed, now it is a string world
}


/*

When functions have references as parameters instead of the actual values, 
we won’t need to return the values in order to give back ownership, 
because we never had ownership.

We call having references as function parameters borrowing.
*/
fn a_borrowing_function(s: &String) { // s is a reference to a String
    println!("Here we have the string {}", s);
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

/*
It does not work. References are immutables by default
*/
fn change(s: &String) {
    some_string.push_str(", world");
}


fn change_with_mut(s: &mut String) {
    some_string.push_str(", world");
}