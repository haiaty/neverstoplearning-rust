/* 

install
web3 = { git = "https://github.com/tomusdrw/rust-web3" }
tokio = { version = "0.2", features = ["full"] }


install ganache:
https://www.trufflesuite.com/ganache


*/

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::Http::new("http://127.0.0.1:7545")?;
    //let transport = web3::transports::Http::new("https://kovan.infura.io")?;
    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    
    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}