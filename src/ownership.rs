

fn main {

    //===========
    // INFO
    //==========

    // - it enables Rust to make memory safety guarantees without needing a garbage collector
    // - memory is managed through a system of ownership with a set of rules that the compiler checks at compile time

    /*
    Pushing to the stack is faster than allocating on the heap because the operating system never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work, because the operating system must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
    Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory.
    By the same token, a processor can do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap). Allocating a large amount of space on the heap can also take time.
    */

    //===========
    // OWNERSHIP RULES
    //================

    // - Each value in Rust has a variable that’s called its owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.
}