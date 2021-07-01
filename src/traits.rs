

fn main () {


    /**
    - functionality a particular type has and can share with other types
    -   We can use traits to define shared behavior in an abstract way
    */

    trait SoundEmitter {
        fn emitSound(&self);

        fn defaultImpl() {

            println!("default");
        }

        fn defaultImplTwo(&self) {

            //ATTENTION: you can't use struct fields inside a default trait implementation.
            // if you to do something like this :  self.name
            // in this default implementation, you will get an error.
            // Default implementations can only  use methods that are defined on the trait or in a super trait.
            println!("default");
        }

        //ATTENTION: associated functions can't be inserted in traits
    }


    struct Duck {

        name: String
    }

    struct Lion {
        name: String
    }

    impl SoundEmitter for Lion {

        fn emitSound(&self) {
            println!("Roarr");
        }
    }

    let l = Lion {name: "name".into()}:

    //================
    // Trait as function parameters
    //================
    
    makeAnimalEmitASound(l);

    //================
    // Trait bounds
    //================
    /*
    - We can use trait bounds to specify that a generic can be any type that has certain behavior
    */

    //- We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets.
    
    //NOTE: this is the same as the fn makeAnimalEmitASound(animal : impl SoundEmitter)
    
    fn test<T: SoundEmitter>(animal: T) {
        animal.emitSound();
    }
}


fn makeAnimalEmitASound(animal : impl SoundEmitter) {
    animal.emitSound();
}