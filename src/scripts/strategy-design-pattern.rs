/**
 * An example of the "strategy" design pattern in Rust ( for info about strategy pattern: https://sourcemaking.com/design_patterns/strategy/php)
 * 
 */

use std::env;

// trait (like an "interface" in this case)
trait Execution {

    fn execute(&self);
}

// struct  StrategyA with trait implementation 
// object oriented: "object StrategyA" that implements the Execution "interface"
struct StrategyA {}

impl Execution for StrategyA {

    fn execute(&self) {
        println!("Executing strategy A")
    }
}
// struct  StrategyB with trait implementation 
// object oriented: "object StrategyB" that implements the Execution "interface"
struct StrategyB {}

impl Execution for StrategyB {
    fn execute(&self) {
        println!("Executing strategy B")
    }
}



fn main() {
    
    let args: Vec<String> = env::args().collect();
  
    let strategy : &str= args[1].as_ref();

    println!("{:?}", strategy);


    // A generic type parameter can only be substituted with one concrete type at 
    // a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.

    // strategy : Box<dyn Execution> --> in this way we are telling that the strategy variable is a type that implements the trait 'Execution'
    let strategy : Box<dyn Execution> = match strategy {
        "A" => Box::new(StrategyA{}),
        "B" => Box::new(StrategyB {}),
        _ => panic!("strategy not valid. consider passing A or B")
    };

    strategy.execute();
   
    run_statregy(strategy);
}

//// <T: ?Sized> -- means that function can be used with both sized and unsized T
/// where T: Execution -- means that the type must implment the trait 'Execution'
fn run_statregy<T: ?Sized>(arg: Box<T>) where T: Execution  {
    arg.execute();
}
