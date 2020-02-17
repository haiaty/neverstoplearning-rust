


fn main() {

    // - Enums allow you to define a type by enumerating its possible variants
    // - we can enumerate all possible variants, which is where enumeration gets its name.

    // examples

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    //enums can be used to define struct types
    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    
    println!("enum four {:#?}", four);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("enum {:#?}", home);

    // using enum concise form to store values

    //We can represent the same concept in a more concise way using just an enum, 
    //rather than an enum inside a struct
    enum IpAddr2 {
        V4(String),
        V6(String),
    }

    let home = IpAddr2::V4(String::from("127.0.0.1"));

    let loopback = IpAddr2::V6(String::from("::1"));

    // using enum can have different types and amounts of associated data
    //There’s another advantage to using an enum rather than a struct: 
    //each variant can have different types and amounts of associated data. 

    #[derive(Debug)]
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr3::V4(127, 0, 0, 1);

    let loopback = IpAddr3::V6(String::from("::1"));

    println!("loopback is {:#?}", loopback);

    //This code illustrates that you can put any kind of data inside an enum variant: 
    //strings, numeric types, or structs, for example.
    struct Ipv4Addr {
        // --snip--
    }
    
    struct Ipv6Addr {
        // --snip--
    }
    
    enum IpAddr5 {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }


    /*
    this one has a wide variety of types embedded in its variants. 
    This enum has four variants with different types:
    */ 
    #[derive(Debug)]
    enum Message {
        Quit, //Quit has no data associated with it at all.
        Move { x: i32, y: i32 }, //Move includes an anonymous struct inside it.
        Write(String), //Write includes a single String.
        ChangeColor(i32, i32, i32), //ChangeColor includes three i32 values.
    }

    //just as we’re able to define methods on structs using impl, 
    //we’re also able to define methods on enums
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    
    let m = Message::Write(String::from("hello"));
    m.call();

    println!("Message is {:#?}", m)

}