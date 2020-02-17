
#[derive(Debug)]
struct File {
    path : String,
    dimension: u32
}

impl File {

    // associated methods don't use the &self, the reference
    // of the instance 
    // often they are used as static constructors
    fn from(path: String, dimension: u32) -> File {
        File {
            path,
            dimension
        }
    }
}

fn main() {

    // associated methods are kind of "static methods" in OO.
    // they do not use the &self

    let file = File::from(String::from("/path"), 10);

    println!("File created {:#?}", file);

}