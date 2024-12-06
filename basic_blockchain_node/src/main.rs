mod block;
mod blockchain;
mod transaction;
mod network;

use blockchain::Blockchain;
use transaction::Transaction;
use network::Network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut blockchain = Blockchain::new();
    let mut network = Network::new().await?;

    let transaction = Transaction::new(
        
        "Alice".to_string(),
        "Bob".to_string(),
        100,
    );

    blockchain.add_block(vec![transaction]);

    network.run().await;

    Ok(())
}