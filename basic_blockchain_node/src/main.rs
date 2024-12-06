mod block;
mod blockchain;
mod transaction;
mod network;

use blockchain::Blockchain;
use transaction::Transaction;
use network::Network;

fn main() {
    let mut blockchain = Blockchain::new();
    
    blockchain.add_block(String::from("First Block"));
    blockchain.add_block(String::from("Second Block"));
    blockchain.add_block(String::from("Third Block"));

    for block in blockchain.chain.iter() {
        println!("{:?}", block);
    }

    println!("Blockchain is valid: {}", blockchain.is_valid());
}