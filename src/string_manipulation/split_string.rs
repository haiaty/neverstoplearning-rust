


fn main() {
    //================
    //By separator:
    //=============

    // s.split("separator")  |  s.split('/')  |  s.split(char::is_numeric)

    let a = "string.pp".split(".");

    //This gives an iterator, which you can loop over...

    for s in split {
        println!("{}", s)
    }

    //....or collect() into a vector.
    let vec = split.collect::<Vec<&str>>();
    // OR
    let vec: Vec<&str> = split.collect();

    //===============
    //By whitespace:
    //===============

         // s.split_whitespace()

    //===============
    //By newlines:
    //=============

         //s.lines()

    //====================
    //By regex: (using regex crate)
    //=================

        //Regex::new(r"\s").unwrap().split("one two three")
}