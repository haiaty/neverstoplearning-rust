/**
https://doc.rust-lang.org/book/ch09-00-error-handling.html


*/

// wITHOUT PANIC BUT USING RESULT 
// return a Result from the function
// which contains OK in case of success  and Err in case of failure.
//
// These cases can either be explicitly handled via match or implicitly with unwrap. Implicit handling will either return the inner element or panic.
// example:
pub fn run(action: String) -> Result<String, String>{

    let allowed_actions = ["SWAP", "TRANSFER"];

    if( ! allowed_actions.contains(&&*action) ) {
        Err("Action not allowed".to_string())
    } else {
        Ok("Action allowed".to_string())
    }
  
  // ex client code: run("OOO".to_string()).unwrap();
  // ex client code: run("OOO".to_string()).expect("not allowed");
  
  // using match
  
  let check_result = run("OOO".to_string());
   let result = match check_result {
         Ok(file) => file,
         Err(error) => panic!("{}", error) //or do whatever you want,
     };
  
  
  
  //============
  // WITH PANIC
  //==============
  // 
  // when use panic? when you don't want to handle the error and it's really a critical error in the application
  fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" { 
      panic!("AAAaaaaa!!!!"); 
  }

    println!("Some refreshing {} is all I need.", beverage);
}
