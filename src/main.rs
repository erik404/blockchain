use crate::config::Config;
use crate::core::blockchain::Blockchain;
use crate::structs::transaction::Transaction;

pub mod tests {
    pub mod helpers;
}

pub mod errors {
    pub mod transaction_errors;
}

pub mod config;

pub mod core {
    pub mod block;
    pub mod blockchain;
}

pub mod structs {
    pub mod token;
    pub mod transaction;
}
fn main() {
    let config: Config = Config::load();
    let mut blockchain: Blockchain = Blockchain::new(config).unwrap();

    blockchain
        .accounts
        .insert("Alice".to_string(), 1_000_000_000); // 10 LTK
    blockchain.accounts.insert("Bob".to_string(), 2_500_000_000); // 25 LTK


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
        "Bob".to_string(), "Charlie".to_string(), 30
    ));

    blockchain.mempool.push(Transaction::new(
        "Bob".to_string(), "Bob".to_string(), 30
    ));

    blockchain.add_block();

    // Print the blockchain and account balances
    println!("Blockchain: {:#?}", blockchain);

    let tx_history = blockchain.get_transaction_history(&"Alice".to_string());
    println!("{:#?}", tx_history);

}
