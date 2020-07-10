// reference: https://refactoring.guru/design-patterns/factory-method
struct ProductA {}

struct ProductB {}

trait Product {
    fn doStuff(&self);
}

impl Product for ProductA {
    
    fn doStuff(&self) { 
        println!("Doing stuff on ProductA"); 
    }
}

impl Product for ProductB {

    fn doStuff(&self) { 
        println!("Doing stuff on ProductB");
    }
}

trait Creator {

    // Box<dyn Product> means that we are returning an 
    // struct/enum/ that implements the trait 'Product'
    // We use Box (https://doc.rust-lang.org/std/boxed/index.html) because it will be allocated in the heap and
    // in order to compile rust need to know the size. because it's a pointer that has the same number of bytes as two usize elements, so it's well-known how to return it.
    fn createProduct() -> Box<dyn Product>;

}

struct ConcreteCreatorA {}

struct ConcreteCreatorB {}

impl Creator for ConcreteCreatorA {

    fn createProduct() -> Box<dyn Product> {
        Box::new(ProductA {})
    }

}

impl Creator for ConcreteCreatorB {
    fn createProduct() ->  Box<dyn Product> {
        Box::new(ProductB {})
    }
}


fn main() {
    

    ConcreteCreatorA::createProduct().doStuff();

    ConcreteCreatorB::createProduct().doStuff();

    



}

