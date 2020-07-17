

fn main () {
     let aCircle = Circle { radius: 3} ;

     println!("Cloning...");
     let anotherCircle = aCircle.clone();

     //println!("{:#?}", anotherCircle);
}


trait Shape {
    fn clone(&self) -> Box<dyn Shape>;
}

#[derive(Debug)]
struct Circle {
    radius : u32
}

impl Shape for Circle {

    fn clone(&self) -> Box<dyn Shape>{
        Box::new(Self{
            radius : self.radius
        })
    }
}


struct Rectangle {
    width: u32,
    height: u32
}

impl Shape for Rectangle {

    fn clone(&self) -> Box<dyn Shape> {
        Box::new(Self{
            width: self.width,
            height: self.height
        })
    }
}