
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

pub mod block;
mod blockchain;
mod transaction;

fn main() {

    let mut blockchain: Blockchain = Blockchain::new(4);

    blockchain.mempool.push(Transaction::new("Transaction 1".to_string()));
    blockchain.mempool.push(Transaction::new("Transaction 2".to_string()));
    blockchain.mempool.push(Transaction::new("Transaction 3".to_string()));
    blockchain.mempool.push(Transaction::new("Transaction 4".to_string()));

    blockchain.add_block();

    blockchain.mempool.push(Transaction::new("Transaction 5".to_string()));
    blockchain.mempool.push(Transaction::new("Transaction 6".to_string()));
    blockchain.mempool.push(Transaction::new("Transaction 7".to_string()));
    blockchain.mempool.push(Transaction::new("Transaction 8".to_string()));

    blockchain.add_block();

    for block in blockchain.chain.iter() {
        println!("{:#?}", block);
    }
}
