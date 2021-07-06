


fn main() {


    let tokens = vec!([1,2]);

    eprintln!("tokens = {:#?}", tokens);
    eprintln!("tokens = {:?}", tokens);

    println!("Value {:#?}", tokens);
    println!("Value: {}", tokens);

    process::exit(0x0100);

}