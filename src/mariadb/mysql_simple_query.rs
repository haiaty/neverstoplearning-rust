
// new API
// mysql=18.2.0

use mysql::*;
use mysql::prelude::*;

fn main() -> Result<()> {

    // create the pool
    let pool = mysql::Pool::new("mysql://root:rootdb@localhost:3301/test")?;

    // get the connection
    let mut conn = pool.get_conn()?;
   
   

    /*
    ==============
    using  .fetch(conn)
    ========
    a way to get results from a table
    */
    let result: Vec<String> = "SELECT table_name FROM information_schema.tables".fetch(conn)?;

    for row in result.iter() {

        println!("{:#?}", row);
    }




    Ok(())
}
