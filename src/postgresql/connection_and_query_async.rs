/*

.env
DATABASE_URL=postgres://postgres:postgres@localhost/daml


cargo.toml

dotenv = "0.15.0"
serde_json = "1.0.95"
tokio = { version = "1.14.0", features = ["full"] }
tokio-postgres = { version = "0.7.8" }


*/


use std::env;
use tokio_postgres::{Client, NoTls, Row};

pub async fn run(query: String) -> Result<Vec<Row>, Box<dyn std::error::Error>>{

    // Get the database URL from the environment variables
    let database_url = env::var("DATABASE_URL")?;

    println!("{}", database_url);

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // Spawn a task to handle the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Execute a query to get the list of tables in the public schema
    let rows = client
        .query(&query, &[])
        .await?;


    Ok(rows)


}
