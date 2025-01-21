
use crate::blockchain::Blockchain;

pub mod block;
mod blockchain;

fn main() {
    // let mut blockchain = Blockchain::new();
    let mut blockchain = Blockchain::new(4); // Set difficulty to 4

    blockchain.add_block(vec!["Block 1: First transaction".to_string(), "Block 1: Second transaction".to_string(), "Block 1: Second transaction".to_string()]);
    blockchain.add_block(vec!["Block 2: First transaction".to_string(), "Block 2: Second transaction".to_string()]);
    blockchain.add_block(vec!["Block 3: First transaction".to_string()]);


    // Print the blockchain
    for block in blockchain.chain.iter() {
        println!("{:#?}", block);
    }

    println!("Original Blockchain Valid: {}", blockchain.is_valid());

}
