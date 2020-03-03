
//FROM the rust by example online book: https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html


#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    println!("rectangle {:#?}", _rectangle);


    println!("The rectangle area is: {}", rect_area(_rectangle));



    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

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

/*

Activity 1 implementation

*/
fn rect_area(rect: Rectangle) -> f32 {
    let top_left_y = rect.top_left.y;

    let bottom_right_x = rect.bottom_right.x;

    top_left_y * bottom_right_x
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