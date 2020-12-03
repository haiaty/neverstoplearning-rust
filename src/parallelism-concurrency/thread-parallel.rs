use std::thread;
use std::time::Duration;

// - threads run multiple different tasks at once (simultaneously, at the same time)
// - there’s no inherent guarantee about THE ORDER in which parts of your code on different threads will run
// - the Rust standard library only provides an implementation of 1:1 threading (when you run thread::spawn it is actually one thread in the Operating system, it is diffferent from green threads, like Go, where many threads can run on a single Os thread)
//     - but there are crates that implements M:N threading
fn main() {

    //NOTE: The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads

    // To create a new thread, we call the thread::spawn function and pass it a closure
    // in this case the return type is JoinHandle.
    //  A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish.
    let thread_one = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the first spawned thread!", i);

            //The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run
            thread::sleep(Duration::from_millis(1));
        }
    });

    let thread_two = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the second spawned thread!", i);


            thread::sleep(Duration::from_millis(1));
        }
    });

    // Wait for both threads to complete.
    // first we call join() in the JoinHandle of the first thread to wait it to finish.
    // NOTE: Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates.
    thread_one.join().expect("thread one panicked");
    // then we call join() in the JoinHandle of the second spawned thread to wait it to finish
    thread_two.join().expect("thread two panicked");
}