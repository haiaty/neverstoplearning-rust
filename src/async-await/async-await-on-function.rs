use std::error::Error;
/*

Note: we return Result<(), Box<dyn Error> if we have different kind of errors
while, if we know that the result would be of a given type we could use for example : Result<(), reqwest::Error>. 

This is because of propagating error . see:https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html


NOTE: since we use await we should declare the function async
*/
pub async fn create_new_field() -> Result<(), Box<dyn Error>> {


    let client = reqwest::Client::new();


    let res = client.post("http://tools.francescozanoni.it/http/request.php")
        .body("the exact body that is sent")
        .send()
        .await?; // the ? means that if there is any error, it will be propagated

    println!("res status: {:#?}", res.status());
    println!("res status: {:#?}", res.text().await?);


    Ok(()) // since this return void, we should wrap void type () into an OK enum


}



//the main of code above

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

   create_new_field().await;

   Ok(())

}



