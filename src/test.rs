
   trait SoundEmitter {
    fn emitSound(&self);
    fn defaultImpl(&self) {
        println!("default");
    }
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

fn main() {

   /*let x : String  = my_func(1u32);

   println!("here it is x: {}", x);*/



let l = Lion {};

makeAnimalEmitASound(l);

let d = Duck {};


let sh = say_hello;

sh();


 //
 let mut three = 3; //simply a mutable variable
 {
 let plus_three = | x | { x + three};

 println!("3 + 3 = {}" , plus_three(3));
 }

 let borrow_three = &mut three;






}


fn say_hello() {
    println!("hello");
}

fn makeAnimalEmitASound(animal : impl SoundEmitter) {
    animal.emitSound();
}


/*
fn my_func<T>(x : u32) -> T {

    
    match x {
        1 => String::from("Hello"),
        _ => 2
    }


}

*/

