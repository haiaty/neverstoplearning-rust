


fn main() {

    // An important thing to keep in mind with the Rust replace() function is that it acts upon every instance.
    let animal = "cat";

    // Replace a string-slice with another in the string.
    // ... A new string is returned.
    let result = animal.replace("at", "alf");
    println!("REPLACED: {}", result);

    /*
    If we only want to replace the first matching instance of a pattern, we can use replacen. Pass the argument 1 to indicate "first only."
   Tip Other numbers will also have the expected result—for example, 2 will modify 2 matches only.
    */

    let source = "ABCABC";

    // Use replacen and specify 1 as the third argument.
    // ... This replaces only the first match.
    let replace_first = source.replacen("BC", "_", 1);
    println!("FIRST MATCH ONLY: {0}", replace_first);
}