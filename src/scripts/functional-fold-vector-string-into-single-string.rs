


fn main () {

let vec : Vec<&str> = ["hello", "world"]; 

/**
NOTES

the keyword 'mut' is put before the two params

*/
let a = vec.iter().fold("".to_string(), |mut total, next| {
        total.push_str(&format!("\" {} \",", next));
        total
    });


}
