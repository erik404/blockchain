use crate::structs::transaction::Transaction;
use chrono::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(
        index: u32,
        transactions: Vec<Transaction>,
        previous_hash: String,
        difficulty: usize,
    ) -> Self {
        let timestamp: String = Utc::now().to_rfc3339();
        let mut block: Block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = Block::calculate_hash(
            index,
            &block.timestamp,
            &block.transactions,
            &block.previous_hash,
            block.nonce,
        );
        block.mine(difficulty);
        block
    }

    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
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

    pub fn calculate_hash(
        index: u32,
        timestamp: &str,
        transactions: &Vec<Transaction>,
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        let input = format!(
            "{}{}{}{}{}",
            index,
            timestamp,
            Self::transactions_string(transactions),
            previous_hash,
            nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        hex::encode(hasher.finalize())
    }

    fn transactions_string(transactions: &Vec<Transaction>) -> String {
        let mut transactions_string: String = String::new();
        for transaction in transactions {
            transactions_string.push_str(transaction.stringify().as_str());
        }
        transactions_string
    }
}
