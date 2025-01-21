use crate::block::*;
use crate::transaction::Transaction;

#[derive(Debug)]
pub(crate) struct Blockchain {
    pub(crate) chain: Vec<Block>,
    pub(crate) mempool: Vec<Transaction>,
    difficulty: usize,
}

impl Blockchain {
    // Create a new blockchain with a genesis block
    pub(crate) fn new(difficulty: usize) -> Self {
        let genesis_block: Block = Block::new(
            0,
            vec![Transaction::new("Genesis Block".to_string())],
            "0".to_string(),
            difficulty,
        );
        Blockchain {
            chain: vec![genesis_block],
            mempool: vec![],
            difficulty,
        }
    }

    // Add a new block to the blockchain
    pub(crate) fn add_block(&mut self) {
        let valid_transactions: Vec<Transaction> = self.process_mempool();
        let last_block: &Block = self.chain.last().unwrap();

        let new_block: Block = Block::new(
            last_block.index + 1,
            valid_transactions,
            last_block.hash.clone(),
            self.difficulty,
        );

        self.chain.push(new_block);
        self.is_valid();
    }

    fn process_mempool(&mut self) -> Vec<Transaction> {
        let mut valid_transactions: Vec<Transaction> = vec![];
        for transaction in &self.mempool {
            if let Ok(_) = self.validate_transaction(&transaction) {
                // TODO validate logic using Account Model
                valid_transactions.push(transaction.clone());
            } else {
                eprintln!("{:#?} is not valid!", transaction);
            }
        }
        self.mempool.clear();
        valid_transactions
    }

    fn validate_transaction(&self, _transaction: &Transaction) -> Result<(), String> {
        // // Ensure sender has enough balance
        // if let Some(balance) = self.balances.get(&transaction.sender) {
        //     if *balance < transaction.amount {
        //         return Err(format!(
        //             "Insufficient balance: {} has {} but tried to send {}",
        //             transaction.sender, balance, transaction.amount
        //         ));
        //     }
        // } else {
        //     return Err(format!(
        //         "Sender {} does not have an account",
        //         transaction.sender
        //     ));
        // }
        //
        // // Ensure the transaction amount is positive
        // if transaction.amount == 0 {
        //     return Err("Transaction amount must be greater than zero".to_string());
        // }

        Ok(())
    }

    // Verify the integrity of the blockchain
    pub(crate) fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block: &Block = &self.chain[i];
            let previous_block: &Block = &self.chain[i - 1];

            // Check if the hashes match
            if current_block.previous_hash != previous_block.hash {
                eprintln!("Chain is broken at block {}!", current_block.index);
                return false;
            }

            // Check if the current block's hash is valid
            let recalculated_hash: String = Block::calculate_hash(
                current_block.index,
                &current_block.timestamp,
                &current_block.transactions,
                &current_block.previous_hash,
                current_block.nonce,
            );
            if current_block.hash != recalculated_hash {
                eprintln!("Block {} has invalid hash!", current_block.index);
                return false;
            }
        }
        println!("{} blocks are valid.", self.chain.len());
        true
    }
}
