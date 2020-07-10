/**
 [dependencies]
anyhow = "1.0"
tokio = { version = "0.2", features = ["full"] }
sqlx = { version = "0.3", default-features = false, features = [ "mysql",  "runtime-tokio", "macros" ] }
dotenv = "0.15.0"


MORE EXAMPLES:
https://github.com/launchbadge/sqlx/blob/master/examples/mysql/todos/src/main.rs

RESOURCES:
https://github.com/launchbadge/sqlx#usage

 * 
 * 
 */

extern crate dotenv;
extern crate sqlx;

use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlQueryAs;


const ENV_FILE_PATH : &str = ".env";

#[tokio::main]
async fn main() -> anyhow::Result<()> {


    let mut pool = utils::establish_connection_pool_sqlx().await;


    let query = r#"
    SELECT DISTINCT myvalue
    FROM my_table
    WHERE param = ?;
    "#;

    let rec : (String, ) = sqlx::query_as(&query)
                            .bind(code_source)
                            .fetch_one(&pool).await.unwrap();


    OK(())


}


pub async fn establish_connection_pool_sqlx() -> MySqlPool {

    dotenv::from_filename(ENV_FILE_PATH).ok();

    let database_url = dotenv::var("DATABASE_URL").expect("You should define the url of DB");

    let pool = MySqlPool::builder()
                .max_size(5)
                .build(&database_url).await.expect("Failed to open pool");

   pool
}