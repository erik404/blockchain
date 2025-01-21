
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

pub mod block;
mod blockchain;
mod transaction;

fn main() {

    let mut blockchain: Blockchain = Blockchain::new(4);

    // Initialize balances
    blockchain.accounts.insert("Alice".to_string(), 100);
    blockchain.accounts.insert("Bob".to_string(), 50);

    println!("Account Balances: {:#?}", blockchain.accounts);
    
    blockchain.mempool.push(Transaction::new(
        "Alice".to_string(), "Bob".to_string(), 20
    ));

    blockchain.mempool.push(Transaction::new(
        "Bob".to_string(), "Alice".to_string(), 10
    ));

    blockchain.mempool.push(Transaction::new(
        "David".to_string(), "Charlie".to_string(), 10
    ));

    blockchain.add_block();
    
    // Print the blockchain and account balances
    println!("Blockchain: {:#?}", blockchain);

    
}
