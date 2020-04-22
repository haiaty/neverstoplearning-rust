

fn main() {

    // with i32
    let p = Point { x:5, y:6};

    println!("x of p is :  {}", p.a_method());

    //with floating points
    let p = Point { x:5.0, y:6.7};
    println!("x of p is :  {}", p.a_method());

    let p = Point { x: "hello", y: "World"};
    println!("x of p is :  {}", p.a_method());


    

   // let number_list  = vec![1, 2, 3];

    //println!("The largest number is {}", largest(&number_list));

}

/*

========================
In FUNCTION DEFINTIONS
=======================


When defining a function that uses generics, we place the generics in
 the signature of the function where we would usually specify the data types 
 of the parameters and return value

 We read this definition as: the function largest is generic over some type T. 
 This function has one parameter named list, which is a slice of values of type T.
 The largest function will return a value of the same type T.
 */
/*
fn largest<T>(number_list : &[T]) -> T {

    let mut largest = number_list[0];

    for &number in number_list {
        if number > largest {
            largest = number;
        }
    };

    largest

}
*/
/*
========================
In STRUCT DEFINTIONS
=======================

- First, we declare the name of the type parameter inside angle brackets just after 
the name of the struct. Then we can use the generic type in the struct definition
 */

 struct Point<T> {
     x: T,
     y: T
 }

 //=== Multiple generic type ====

 struct PointTwo<T, U> {
    x: T,
    y: U
 }

 /*
========================
In ENUM DEFINTIONS
=======================
*/
#[derive(Debug)]
 enum MyOption<T> {
     Some(T),
     None
 }
 //=== Multiple generic types ====


enum ResultExample<T, E> {
    Ok(T),
    Err(E)
}

/*
========================
In METHOD DEFINTIONS
=======================

we have to declare T just after impl so we can use it to specify 
that we’re implementing methods on the type Point<T>
*/

impl<T> Point<T> {

    fn a_method(&self) -> &T {
        &self.x
    }

}


impl<T, U> PointTwo<T, U> {
    fn mixup<V, W>(self, other: PointTwo<V, W>) -> PointTwo<T, W> {
        PointTwo {
            x: self.x,
            y: other.y,
        }
    }
}

/*
We could, for example, implement methods only on Point<f32> instances
 rather than on Point<T> instances with any generic type.

This code means the type Point<f32> will have a method named distance_from_origin 
and other instances of Point<T> where T is not of type f32 will not have this 
method defined.

 */

impl Point <f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

}