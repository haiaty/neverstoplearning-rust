use std::env;
// reference: https://refactoring.guru/design-patterns/abstract-factory

trait Furnitures {

    fn createChair(&self) -> Box<dyn Chair>;
    fn createSofa(&self) -> Box<dyn Sofa>;
}

struct ModernFurnitures {}

impl Furnitures for ModernFurnitures {

    fn createChair(&self) -> Box<dyn Chair> {
        Box::new(ModernChair {})
    }

    fn createSofa(&self) -> Box<dyn Sofa> {
        Box::new(ModernSofa {})
    }
}

struct VictorianFurnitures {}

impl Furnitures for VictorianFurnitures {
    fn createChair(&self) -> Box<dyn Chair> {
        Box::new(VictorianChair {})
    }

    fn createSofa(&self) -> Box<dyn Sofa> {
        Box::new(VictorianSofa {})
    }
}


 //interfaces for a set of distinct but related products which make up a product family.

trait Chair {
    fn printStyle(&self);
}

struct ModernChair {}

impl Chair for ModernChair {
    fn printStyle(&self) {
        println!("Modern chair");
    }
}

struct VictorianChair {}

impl Chair for VictorianChair {

    fn printStyle(&self) {
        println!("Victorian chair");
    }
}

trait Sofa {
    fn printStyle(&self);
}

struct ModernSofa {}

impl Sofa for ModernSofa {
    fn printStyle(&self) {
        println!("Modern sofa");
    }
}

struct VictorianSofa {}

impl Sofa for VictorianSofa {

    fn printStyle(&self) {
        println!("Victorian sofa");
    }
}


fn main() {

    // abstract factory let you work with an object (the "abstract factory")
    // that allows you to create a "family" of related objects

    // once you have established wich kind of family of related things (in this example
    // the furniture style that groups several different products with that style)
    // you can use it in your client code 

    // So it should be used only when you have different families of objects
    // and your client code should work with all of them in a transparent/abstract 
    // way (in other words the client code should not be coupled with the a concrete family)

    let args: Vec<String> = env::args().collect();

    let furnitureStyle : &str= args[1].as_ref();

    println!("{:?}", furnitureStyle);

    let furnitures : Box<dyn Furnitures> = match furnitureStyle {
        "Victorian" => Box::new(VictorianFurnitures{}),
        "Modern" => Box::new(ModernFurnitures {}),
        _ => panic!("furniture style not available. Try Modern or Victorian")
    };

// Notw how the client code work with the abstract factory to create specific products
// that will have same interface per type.
let chair = furnitures.createChair();
let sofa = furnitures.createSofa();

chair.printStyle();
sofa.printStyle();

}

