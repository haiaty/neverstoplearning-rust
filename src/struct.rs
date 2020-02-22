
#[derive(Debug)] //this let the struct to be formated in output of println!
struct Person {
    name : String, // a field of type String
    age: u32
}

// methods of a struct
impl Person {
    fn say_name(&self){
        println!("My name is : {}", self.name);
    }

    fn is_same_name(&self, name: String) -> bool {
        true
    }
}


/*
 Tuple structs are useful when you want to give the whole tuple a name
  and make the tuple be a different type from other tuples, 
  and naming each field as in a regular struct would be verbose or redundant.
*/
struct Color(i32, i32, i32);

/*
You can also define structs that don’t have any fields! 
These are called unit-like structs because they behave similarly to (), 
the unit type. Unit-like structs can be useful in situations in which you need 
to implement a trait on some type but don’t have any data that you want to store in the type itself.
*/


fn main() {

    //we create an instance of that struct by specifying concrete values 
    //for each of the fields. We create an instance by stating the name 
    //of the struct and then add curly brackets containing key: value pairs, 
    //where the keys are the names of the fields and the values are the data 
    //we want to store in those fields
    let a_person = Person { 
        name: String::from("a_name"),
        age: 32
    };

    //We don’t have to specify the fields in the same order 
    //in which we declared them in the struct.

    let another_person = Person {
        age: 20,
        name: String::from("another_name")
    };

    // To get a specific value from a struct, we can use dot notation. 

    let a_name = &a_person.name;

    println!("the name is {}", a_name);

    // to call a method on struct instance

    a_person.say_name();

    a_person.is_same_name(String::from("a name"));


    // to make it writable we need to declare it as mutable  
    let mut a_mutable_struct = Person {
        name: String::from("name"),
        age: 25
    };


    a_mutable_struct.name = "changed Name".to_string();

    println!("the name changed is {}", a_mutable_struct.name);


    //take a struct from a function
    let a_person_from_func = build_person(String::from("name"), 35);

    println!("the name is {} and the age is {}", a_person_from_func.name, a_person_from_func.age);


    //It’s often useful to create a new instance of a struct that uses 
    //most of an old instance’s values but changes some. 
    //You’ll do this using struct update syntax.
    //The syntax .. specifies that the remaining fields not explicitly set should have the same value 
    //as the fields in the given instance.
    let a_person_created_with_struct_update_syntax = Person{
        name: String::from("other"),
        ..a_person
    };


    println!("print a a_person_created_with_struct_update_syntax {:#?}", a_person_created_with_struct_update_syntax);



    // You can also define structs that look similar to tuples, called tuple structs
    // they don’t have names associated with their fields; rather, they just have the types of the fields. 
    let black = Color(0,0,0);

    /*
    tuple struct instances behave like tuples:
     you can destructure them into their individual pieces, you can use a . followed by the index to access an individual value, and so on.
    */
    let r = black.0;

    




}

// we can return a struct as expression
fn build_person(name: String, age: u32) -> Person {
    Person {
        name: name,
        age: age
    }
}


// we can use he field init shorthand to use direclty params in struct
fn build_person_2(name: String, age: u32) -> Person {
    Person {
        name, 
        age
    }
}