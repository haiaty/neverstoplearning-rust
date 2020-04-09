
//mysql = "*"

use mysql::params;

#[derive(Debug, PartialEq, Eq)]
struct MyStructureRecord {
    always_present: Option<String>,
    allowed_values: Option<String>
}


fn main() {

 let pool = mysql::Pool::new("mysql://root:rootdb@localhost:3306/mydb").unwrap();

    let all_records: Vec<MyStructureRecord> =
        pool.prep_exec("
        SELECT always_present, allowed_values
        FROM my_schema.my_table
        WHERE platform=:param", params! {
            "param" => "A PARAM",
        })
 
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (always_present, allowed_values) = mysql::from_row(row);

                    MyStructureRecord {
                        always_present,
                        allowed_values
                    }
                }).collect()
            }).unwrap(); // Unwrap `Vec<MyStructureRecord>`

}
