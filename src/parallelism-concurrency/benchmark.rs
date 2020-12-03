
use std::thread;
use std::time::Instant;

#[tokio::main]
async fn main() {

    //===================
    // using thread
    //======================
    let now = Instant::now();
    let thread_one= thread::spawn(||  {
        let body = reqwest::blocking::get("https://www.rust-lang.org").unwrap()
            .text().unwrap();
        //println!("text: {} ", body);
    });

    thread_one.join().expect("error on thread");
    let elapsed = now.elapsed();
    println!("thread with blocking reqwest Elapsed: {} ms", elapsed.as_millis());

    //===================
    // async-await
    //======================
    let now = Instant::now();
    let body = reqwest::get("https://www.rust-lang.org").await.unwrap()
        .text().await.unwrap();
    let elapsed = now.elapsed();
    println!("async reqwest using tokio Elapsed: {} ms", elapsed.as_millis());

   // println!("file to download: '{}'", body);

}