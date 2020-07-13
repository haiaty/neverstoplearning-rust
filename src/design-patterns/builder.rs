/**
 * 
 * Builder is a creational design pattern that lets you construct complex objects step by step.
 * The pattern allows you to produce different types and representations of an object using the same construction code.
 */

 // reference: https://refactoring.guru/design-patterns/builder

 fn main() {

    let b1 = Box::new(ConcretBuilder1 {});

    let mut director = Director {
        builder: b1
    };

    director.make();
    director.makeAnotherDifferentThing();

    let b2 = Box::new(ConcretBuilder2 {});


    director.changeBuilder(b2);

    director.make();
    director.makeAnotherDifferentThing();



 }

struct Director {
    builder : Box<dyn Builder>
}

impl Director {

    
    fn changeBuilder(&mut self, builder: Box<dyn Builder>) {
        self.builder = builder;
    }

    fn make(&self) {
        self.builder.reset();
        self.builder.buildStepA();

        self.builder.getResult()
    }

    fn makeAnotherDifferentThing(&self) {
        self.builder.reset();
        self.builder.buildStepA();
        self.builder.buildStepB();
        self.builder.getResult()
    }
}

 trait Builder  {
     fn reset(&self);
     fn buildStepA(&self);
     fn buildStepB(&self);
     fn getResult(&self);
 }

 struct ConcretBuilder1 {}

 impl Builder for ConcretBuilder1 {

    fn reset(&self){
        println!("resetting for ConcretBuilder1");
    }
     fn buildStepA(&self){
         println!("doing step A for the concret builder 1");
     }
     fn buildStepB(&self){
        println!("doing step B for the concret builder 1");
     }
     fn getResult(&self){
        println!("getting result for the concret builder 1");
     }
 }


 struct ConcretBuilder2 {}

 impl Builder for ConcretBuilder2 {
    fn reset(&self){
        println!("resetting for ConcretBuilder2");
    }
    fn buildStepA(&self){
        println!("doing step A for the concret builder 2");
    }
    fn buildStepB(&self){
        println!("doing step B for the concret builder 2");
    }
    fn getResult(&self){
        println!("getting result for the concret builder 2");
    }
 }