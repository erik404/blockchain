use crate::structs::transaction::Transaction;
use chrono::prelude::*;
use sha2::{Digest, Sha256};

/// Represents a block in the blockchain.
#[derive(Debug)]
pub struct Block {
    pub index: u32,                     // Block index in the chain
    pub timestamp: String,              // Timestamp when the block was created
    pub transactions: Vec<Transaction>, // List of transactions in the block
    pub previous_hash: String,          // Hash of the previous block
    pub hash: String,                   // Hash of the current block
    pub nonce: u64,                     // Nonce used for mining
}

impl Block {
    /// Creates a new block and mines it to meet the difficulty target.
    pub fn new(
        index: u32,
        transactions: Vec<Transaction>,
        previous_hash: String,
        difficulty: usize,
    ) -> Self {
        let timestamp: String = Utc::now().to_rfc3339(); // Generate a current timestamp
        let mut block: Block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(), // Placeholder for hash
            nonce: 0,            // Start with a nonce of 0
        };

        // Calculate the initial hash
        block.hash = Block::calculate_hash(
            index,
            &block.timestamp,
            &block.transactions,
            &block.previous_hash,
            block.nonce,
        );

        // Mine the block to meet the difficulty target
        block.mine(difficulty);
        block
    }

    /// Mines the block by adjusting the nonce until the hash meets the difficulty target.
    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty); // Target prefix based on difficulty
        while !self.hash.starts_with(&target) {
            self.nonce += 1; // Increment the nonce to find a valid hash
            self.hash = Block::calculate_hash(
                self.index,
                &self.timestamp,
                &self.transactions,
                &self.previous_hash,
                self.nonce,
            );
        }
        println!("Block mined: {}", self.hash);
    }

    /// Calculates the hash of the block based on its properties.
    pub fn calculate_hash(
        index: u32,
        timestamp: &str,
        transactions: &Vec<Transaction>,
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        // Concatenate block data into a single string
        let input = format!(
            "{}{}{}{}{}",
            index,
            timestamp,
            Self::transactions_string(transactions),
            previous_hash,
            nonce
        );

        // Initialize SHA-256 hasher
        let mut hasher = Sha256::new();
        hasher.update(input);
        // Return the hash as a hex string
        hex::encode(hasher.finalize())
    }

    /// Converts the list of transactions into a string for hashing.
    fn transactions_string(transactions: &Vec<Transaction>) -> String {
        let mut transactions_string: String = String::new();
        for transaction in transactions {
            transactions_string.push_str(transaction.stringify().as_str()); // Serialize each transaction
        }
        transactions_string
    }
}
