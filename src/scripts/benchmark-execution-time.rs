use std::time::Instant;


fn main() {
    
    let now = Instant::now();
   
    // code here

    let elapsed = now.elapsed();
    println!("Elapsed: {}", elapsed.as_millis()); //there are other functions such as_second, etc..
   
}
