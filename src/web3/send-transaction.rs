
use web3::types::{TransactionRequest, U256};

#[tokio::main]
async fn main () -> web3::Result<()>{

    let transport = web3::transports::Http::new("http://127.0.0.1:7545")?;
    let web3 = web3::Web3::new(transport);

    let accounts = web3.eth().accounts().await.unwrap();

    let balance_before = web3.eth().balance(accounts[1], None).await.unwrap();
    println!("Balance before: {}", balance_before);

    let tx = TransactionRequest {
        from: accounts[0],
        to: Some(accounts[1]),
        gas: None,
        gas_price: None,
        value: Some(5_000_000.into()),
        data: None,
        nonce: None,
        condition: None
    };

    println!("{:#?}", tx);


    let tx_hash = web3.eth().send_transaction(tx).await.unwrap();

    let balance_after = web3.eth().balance(accounts[1], None).await.unwrap();

    println!("TX Hash: {:?}", tx_hash);
    println!("Balance after: {}", balance_after);



    Ok(())
    
}