use clap::{Arg, App};


/*
clap = "3.0.0-beta.2"
 */
fn main() {
    let matches = App::new("My Program")
        .version("1.0")
        .author("haiaty")
        .about("will be displayed when you run --help on the binary produces")
        // this is how to set input arguments
        // ex usage:    ./target/debug/myprogram "FIRSTARG"
        .arg(Arg::new("FIRSTARG") //the name of the argument is needed in order to take it later
                 .about("the first argument passed on the command line")
                 .required(true)
                 .index(1) //indicates that this is the firs argument
        )
        // this is how to set custom options
        //--source_type <SOURCETYPE>    Sets a custom config file
        // ex usage:    ./target/debug/myprogram --source_type=TYPE
        .arg(Arg::new("source_type")
            .long("source_type")
            .value_name("SOURCETYPE")
            .about("Sets a custom config file")
            .takes_value(true)) //Specifies that the argument takes a value at run time.
        .get_matches();

    if let Some(i) = matches.value_of("FIRSTARG") {
        println!("Value for input: {}", i);
    }
}
