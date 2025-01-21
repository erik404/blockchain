
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

pub mod block;
mod blockchain;
mod transaction;

fn main() {
    // let mut blockchain = Blockchain::new();
    let mut blockchain = Blockchain::new(4); // Set difficulty to 4

    let transaction: Transaction = Transaction { value: "Block 1: First transaction".to_string() };
    let transaction_2: Transaction = Transaction { value: "Block 1: Second transaction".to_string() };
    blockchain.add_block(vec![transaction, transaction_2]);

    let transaction: Transaction = Transaction { value: "Block 2: First transaction".to_string() };
    let transaction_2: Transaction = Transaction { value: "Block 2: Second transaction".to_string() };
    blockchain.add_block(vec![transaction, transaction_2]);


    // Print the blockchain
    for block in blockchain.chain.iter() {
        println!("{:#?}", block);
    }

    println!("Original Blockchain Valid: {}", blockchain.is_valid());

}
