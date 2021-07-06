use std::process;

fn main() {
    println!("something");
    println!("Command that will be executed: {:#?}");
    process::exit(0x0100);
}