

fn main () {

    // the slice of string is of type &str, the slice type of an array of integers is of type &[i32]
    // - does not have ownership
    // - Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection
    // - A string slice is a reference to part of a String
    // - string slice range indices must occur at valid UTF-8 character boundaries.  If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // if you want to start at the first index (zero), you can drop the value before the two periods
    let s = String::from("hello");

    let slice = &s[..2];

    // if your slice includes the last byte of the String, you can drop the trailing number
    let slice = &s[3..];

    //You can also drop both values to take a slice of the entire string
    let slice = &s[..];


    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}


fn first_word(s: &str ) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}