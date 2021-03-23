/*

[dependencies]
reqwest = "0.11.2"
tokio =  { version = "1.4", features = ["full"] }
error-chain = "0.12.4"

 */

use error_chain::error_chain;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;


error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}


#[tokio::main]
async fn main() -> Result<()> {

    //====== DOWNLOAD FILE =========


    let target = "https://www.example.org/some.pdf";

    // Execute the request
    let response = reqwest::get(target).await?;

    // here we are creating a file using the name
    // used in the url, so in this case some.pdf
    let mut dest = {

        // take the name of the file from url
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        // create a new file and return it.
        // NOTE that we've created a scope and are returning from the scope
        // and this file struct will be inside the variable "dest"
        let path = Path::new(&fname);
        File::create(path)?
    };

    // here we taking the response as bytes...
    let content = response.bytes().await?;

    // and since it is a file we write it directly to the created file
    dest.write_all(&content)?;

    Ok(())
}