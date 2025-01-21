use chrono::prelude::*;
use sha2::{Digest, Sha256};
use crate::transaction::Transaction;

#[derive(Debug)]
pub(crate) struct Block {
    pub(crate) index: u32,
    pub(crate) timestamp: String,
    pub(crate) transactions: Vec<Transaction>,
    pub(crate) previous_hash: String,
    pub(crate) hash: String,
    pub(crate) nonce: u64,
}

impl Block {

    pub(crate) fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut block: Block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0
        };

        block.hash = Block::calculate_hash(index, &block.timestamp, &block.transactions, &block.previous_hash, block.nonce);
        block.mine(difficulty); // Mine the block
        block
    }

    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty); // Target hash must start with 'difficulty' number of zeroes
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = Block::calculate_hash(self.index, &self.timestamp, &self.transactions, &self.previous_hash, self.nonce);
        }
        println!("Block mined: {}", self.hash);
    }

    pub(crate) fn calculate_hash(index: u32, timestamp: &str, transactions: &Vec<Transaction>, previous_hash: &str, nonce: u64) -> String {
        let input = format!("{}{}{:#?}{}{}", index, timestamp, transactions, previous_hash, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input);
        hex::encode(hasher.finalize())
    }
}
