/**
 * 

serde_json = "1.0"

 */

fn main() {

use serde_json::{Value};

// example of accessing json Value
//let payloadBase64Encoded : &str =  v["data"].as_str().unwrap();


// generate to json string
   let x: i32 = 5;
    let xs = serde_json::to_string(&x).unwrap();
    println!("i32 number {} serializes into string {}", x, xs);
    
    // take back the value
    let xd: i32 = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);


// 
// OTHER EXAMPLES
//
    let x: f32 = 3.14;
    let xs = serde_json::to_string(&x).unwrap();
    println!("f32 number {} serializes into string {}", x, xs);

    let x: Vec<u8> = vec![1, 2, 3];
    let xs = serde_json::to_string(&x).unwrap();
    println!("Vec<u8> {:?} serializes into string {}", x, xs);
    let xd: Vec<u8> = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);

    let x: Vec<f32> = vec![3.141, 2.718, 1.618];
    let xs = serde_json::to_string(&x).unwrap();
    println!("Vec<f32> {:?} serializes into string {}", x, xs);
    
    let x: (i32, &str, f32, bool) = (1, "hello", 4.5, true);
    let xs = serde_json::to_string(&x).unwrap();
    println!("tuple {:?} serializes into string {}", x, xs);
    let xd: (i32, &str, f32, bool) = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);

    let x = ((1u8, 2u16), (3.141f32, 'a'), true);
    let xs = serde_json::to_string(&x).unwrap();
    println!("nested tuple {:?} serializes into string {}", x, xs);

}
