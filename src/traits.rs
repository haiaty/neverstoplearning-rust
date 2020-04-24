

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

        //ATTENTION: associated functions can't be inserted in traits
    }


    struct Duck {

    }

    struct Lion {

    }

    impl SoundEmitter for Lion {

        fn emitSound(&self) {
            println!("Roarr");
        }
    }

    let l = Lion {}:

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