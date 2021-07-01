use clap::{Arg, App};


/*
clap = "3.0.0-beta.2"
 */
fn main() {
    let matches = App::new("My Program")
        .version("1.0")
        .author("haiaty")
        .about("will be displayed when you run --help on the binary produces")
        .arg(Arg::new("FIRSTARG") //the name of the argument is needed in order to take it later
                 .about("the first argument passed on the command line")
                 .required(true)
                 .index(1) //indicates that this is the firs argument
        )
        .get_matches();

    if let Some(i) = matches.value_of("FIRSTARG") {
        println!("Value for input: {}", i);
    }
}
