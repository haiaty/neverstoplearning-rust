
//[dependencies]
//sqlite = "0.26.0"


use sqlite::State;



fn main() {
    
    let connection = sqlite::open("database/operations.sqlite").unwrap();

    let mut statement = connection
        .prepare("SELECT * FROM mytable")
        .unwrap();



    // iterate over the result set
    // this assumes that the first column of the table is a string
    
    while let State::Row = statement.next().unwrap() {

        println!("string column = {}", statement.read::<String>(0).unwrap());


    }

}
