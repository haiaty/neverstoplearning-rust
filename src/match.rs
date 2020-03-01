


fn main () {


    //let a = Date::from(23);
    /*
    allows you to compare a value against a series of patterns and
     then execute code based on which pattern matches

     values go through each pattern in a match, 
     and at the first pattern the value “fits,” 
     the value falls into the associated code block to be used during execution.
    */ 

    // example

    enum Source {
        SourceA,
        SourceB
    }

    fn value_for_sources(a_source: &Source) -> i32 {

        match a_source {
            Source::SourceA => 1, //an match arm that matches sith value Source::SourceA
            Source::SourceB => 2 /*with each arm is an expression, and the resulting value of the expression in the matching arm is the value that gets returned for the entire match expression.*/ 
        }
    }


    let sourcea = Source::SourceA;

    println!("The value of source a is {}", value_for_sources(&sourcea));

    /* If you want to run multiple lines of code in a match arm, you can use curly brackets.*/

    let a = match sourcea {
        Source::SourceA =>  {
            println!("multiple");
            1 //but would still return the last value of the block, 1
        },
        Source::SourceB => 2
    };

    // 

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let coin = Coin::Quarter(UsState::Alabama);

    let x = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    };
    

}