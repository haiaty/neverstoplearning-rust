
use std::{time};
use rand::Rng;
use futures::future::FutureExt;


async fn test() -> String {

    let mut rng = rand::thread_rng();

    let n2: u64 = rng.gen_range(1, 3);

    let rand_millis = time::Duration::from_secs((n2));
    println!("waiting  for  {} seconds....", n2);
    tokio::time::delay_for(rand_millis);
    


    println!("on test()....");
    String::from("ook")
}

async fn test2() -> String {
    println!("on test2()....");
    String::from("ook2")
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(&song).await;
}

async fn learn_song() -> String {
    println!("learning song");
    "a song".to_owned()
}

async fn sing_song(song: &str) {
    println!("singing song  {}", song);


}

async fn dance() {
    println!("dancing");
}

fn dance_sync() {
    println!("dancing");
}


fn learn_song_sync() -> String {
    println!("learning song");
    "a song".to_owned()
}

fn sing_song_sync(song: &str) {
    println!("singing song  {}", song);


}

 fn learn_and_sing_sync() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song_sync();
    sing_song_sync((&song));
}



use std::time::Instant;

#[tokio::main]
async fn main() {

    println!("EXAMPLE 1 ##############\n");

    let now = Instant::now();
     let f1 = learn_and_sing();
     let f2 = dance();

     futures::join!(f1, f2);

     let elapsed = now.elapsed();
     println!("Elapsed: {}", elapsed.as_millis()); //there are other functions such as_second, etc..


     let now = Instant::now();
    learn_and_sing_sync();
     dance_sync();

     let elapsed = now.elapsed();
     println!("Elapsed: {}", elapsed.as_millis()); //there are other functions such as_second, etc..


     println!("EXAMPLE 2 ##############\n");
    
        let future1 = test();
        let future2 = test2();

        futures::future::join(future1, future2).await;
        
        
        println!("EXAMPLE 3 ##############\n");
   
    

        let mut handle = vec![];
        handle.push(test().boxed());
        handle.push(test2().boxed());

    
        println!("{:#?}", futures::future::join_all(handle).await);
        


        
        
    
    
}