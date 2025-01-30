use crate::config::Config;
use crate::core::blockchain::Blockchain;
use crate::core::transaction::Transaction;
use crate::wallet::wallet::Wallet;

#[cfg(test)]
mod test_utils;

pub mod config;

mod common {
    pub mod calculate_hash;
    pub mod compute_address_from_pub_key;
}

mod core {
    pub mod block;
    pub mod blockchain;
    pub mod token;
    pub mod transaction;
}

mod errors {
    pub mod transaction_errors;
}

mod wallet {
    pub mod wallet;
}

fn main() {
    // test_blockchain();
    test_wallet();
}

fn test_wallet() {
    let wallet = Wallet::new();
    let address = wallet.get_address();

    println!("Address: {:?}", address);

    let mut tx = Transaction::new(address.clone(), "TEST_ADDRESS".to_string(), 100);

    let signature = wallet.sign_transaction(&tx.stringify());
    tx.sign(signature);

    println!("Signature: {:?}", tx.signature);
    println!("Signature Verified: {:?}", tx.verify(&wallet.public_key)); // Now requires public key for verification
}

fn test_blockchain() {
    let config: Config = Config::load().unwrap();
    let mut blockchain: Blockchain = Blockchain::new(config).unwrap();

    blockchain
        .accounts
        .insert("Alice".to_string(), 1_000_000_000); // 10 LTK
    blockchain.accounts.insert("Bob".to_string(), 2_500_000_000); // 25 LTK

    // Initialize balances
    blockchain.accounts.insert("Alice".to_string(), 100);
    blockchain.accounts.insert("Bob".to_string(), 50);

    println!("Account Balances: {:#?}", blockchain.accounts);

    blockchain
        .mempool
        .push(Transaction::new("Alice".to_string(), "Bob".to_string(), 20));

    blockchain
        .mempool
        .push(Transaction::new("Bob".to_string(), "Alice".to_string(), 10));

    blockchain.mempool.push(Transaction::new(
        "Bob".to_string(),
        "Charlie".to_string(),
        30,
    ));

    blockchain
        .mempool
        .push(Transaction::new("Bob".to_string(), "Bob".to_string(), 30));

    blockchain.add_block();

    // Print the blockchain and account balances
    println!("Blockchain: {:#?}", blockchain);

    let tx_history = blockchain.get_transaction_history(&"Alice".to_string());
    println!("{:#?}", tx_history);
}
