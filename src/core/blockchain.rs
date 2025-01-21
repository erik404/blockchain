use crate::config::config::Config;
use crate::core::block::*;
use crate::structs::token::Token;
use crate::structs::transaction::Transaction;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub token: Token,
    pub mempool: Vec<Transaction>,
    pub accounts: HashMap<String, u64>,
    difficulty: usize,
}

impl Blockchain {
    // Create a new blockchain with a genesis block
    pub fn new(config: Config) -> Self {
        let token: Token = Token::new(
            config.token.name.clone(),
            config.token.symbol.clone(),
            config.token.decimals,
            config.token.total_supply,
        );

        let mut accounts: HashMap<String, u64> = HashMap::new();
        accounts.insert("Developer".to_string(), 2_100_000_000_000_000); // Pre-mined 10%

        Blockchain {
            chain: vec![Block::new(
                0,
                vec![Transaction::new("".to_string(), "".to_string(), 0)],
                config.blockchain.genesis_name,
                config.blockchain.difficulty,
            )],
            accounts,
            token,
            mempool: vec![],
            difficulty: config.blockchain.difficulty,
        }
    }

    // Add a new block to the blockchain
    pub fn add_block(&mut self) {
        let valid_transactions: Vec<Transaction> = self.process_mempool(); // todo, think about error handling
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

    // Process the memory pool
    fn process_mempool(&mut self) -> Vec<Transaction> {
        let mut processed_transactions: Vec<Transaction> = vec![];
        for transaction in &self.mempool {
            match self.validate_transaction(transaction) {
                Ok(_) => {
                    // Deduct balance from sender.
                    // Deref the hash entry, or_insert 0 is safe here because validate_transaction passed (checks min balance)
                    *self.accounts.entry(transaction.sender.clone()).or_insert(0) -=
                        transaction.amount;
                    // Add balance to receiver.
                    *self
                        .accounts
                        .entry(transaction.receiver.clone())
                        .or_insert(0) += transaction.amount;
                    // Add the transaction to the vec
                    processed_transactions.push(transaction.clone());
                }
                Err(why) => {
                    eprintln!("{:?}", why);
                }
            }
        }
        self.mempool.clear();
        processed_transactions
    }

    // Validate the transaction. Does not check if receiver exist.
    fn validate_transaction(&self, transaction: &Transaction) -> Result<(), String> {
        // Check if the sender has enough balance
        if let Some(balance) = self.accounts.get(&transaction.sender) {
            if *balance < transaction.amount {
                return Err(format!(
                    "Insufficient balance: {} has {} but tried to send {}",
                    transaction.sender, balance, transaction.amount
                ));
            }
        } else {
            return Err(format!("Sender {} does not exist", transaction.sender));
        }

        // Ensure the amount is greater than 0
        if transaction.amount == 0 {
            return Err("Transaction amount must be greater than zero".to_string());
        }

        Ok(())
    }

    // Verify the integrity of the blockchain
    pub fn is_valid(&self) -> bool {
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

    pub fn display_balances(&self) {
        println!("Account Balances:");
        for (account, balance) in &self.accounts {
            println!(
                "{}: {} {}",
                account,
                self.token.format_amount(*balance),
                self.token.symbol
            );
        }
    }
}
