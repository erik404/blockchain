use crate::common::calculate_hash::calculate_block_hash;
use crate::structs::transaction::Transaction;
use chrono::prelude::*;

/// Represents a block in the blockchain.
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
    /// Creates a new block and mines it to meet the difficulty target.
    pub fn new(
        index: u32,
        transactions: Vec<Transaction>,
        previous_hash: String,
        difficulty: usize,
    ) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };

        block.hash = calculate_block_hash(
            index,
            &block.timestamp,
            &block.transactions,
            &block.previous_hash,
            block.nonce,
        );

        block.mine(difficulty);
        block
    }

    /// Mines the block by adjusting the nonce until the hash meets the difficulty target.
    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = calculate_block_hash(
                self.index,
                &self.timestamp,
                &self.transactions,
                &self.previous_hash,
                self.nonce,
            );
        }
        println!("Block mined: {}", self.hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::transaction::Transaction;
    #[test]
    fn new_block_has_correct_properties() {
        let index = 1;
        let transactions = vec![
            Transaction::new("Alice".to_string(), "Bob".to_string(), 100),
            Transaction::new("Charlie".to_string(), "Dave".to_string(), 50),
        ];
        let previous_hash =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let difficulty = 2;

        let block = Block::new(
            index,
            transactions.clone(),
            previous_hash.clone(),
            difficulty,
        );

        // Verify basic properties
        assert_eq!(block.index, index, "Block index should match");
        assert_eq!(
            block.transactions, transactions,
            "Block transactions should match"
        );
        assert_eq!(
            block.previous_hash, previous_hash,
            "Previous hash should match"
        );
        assert_ne!(block.hash, "", "Block hash should not be empty");
    }
    #[test]
    fn mining_generates_valid_block() {
        // Arrange: Create a block with a low difficulty to ensure quick mining
        let index = 1;
        let transactions = vec![Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            100,
        )];
        let previous_hash =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let difficulty = 2; // Low difficulty for test efficiency

        // Create and mine the block
        let block = Block::new(index, transactions, previous_hash, difficulty);

        // Verify that the block's hash meets the difficulty target
        let target = "0".repeat(difficulty);
        assert!(
            block.hash.starts_with(&target),
            "Block hash should start with {} zeros to meet difficulty",
            difficulty
        );
    }
    #[test]
    fn block_hash_changes_with_nonce() {
        let index = 1;
        let transactions = vec![Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            100,
        )];
        let previous_hash =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let difficulty = 1;

        let mut block = Block::new(index, transactions, previous_hash, difficulty);

        // Store the initial hash and mine again to change the nonce
        let initial_hash = block.hash.clone();
        block.hash = "".parse().unwrap();
        block.mine(difficulty);

        // Assert: Verify that the hash has changed after mining
        assert_ne!(
            block.hash, initial_hash,
            "Hash should change after mining with a different nonce"
        );
    }
    #[test]
    fn deterministic_hash_for_same_properties() {
        let index = 1;
        let transactions = vec![Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            100,
        )];
        let previous_hash =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let timestamp = 1223455678.to_string();

        let mut block1 = Block {
            index,
            timestamp: timestamp.clone(),
            transactions: transactions.clone(),
            previous_hash: previous_hash.clone(),
            hash: String::new(),
            nonce: 0,
        };
        block1.hash = calculate_block_hash(
            index,
            &block1.timestamp,
            &block1.transactions,
            &block1.previous_hash,
            block1.nonce,
        );

        let mut block2 = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block2.hash = calculate_block_hash(
            index,
            &block2.timestamp,
            &block2.transactions,
            &block2.previous_hash,
            block2.nonce,
        );

        // Assert: Verify that both blocks have the same hash (deterministic behavior)
        assert_eq!(
            block1.hash, block2.hash,
            "Blocks with the same properties should have the same hash before mining"
        );
    }
    #[test]
    fn unique_hash_for_different_blocks() {
        // Arrange: Create two blocks with different properties
        let index1 = 1;
        let index2 = 2;
        let transactions = vec![Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            100,
        )];
        let previous_hash1 =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let previous_hash2 =
            "1111111111111111111111111111111111111111111111111111111111111111".to_string();
        let difficulty = 2;

        // Create two blocks with different inputs
        let block1 = Block::new(index1, transactions.clone(), previous_hash1, difficulty);
        let block2 = Block::new(index2, transactions.clone(), previous_hash2, difficulty);

        assert_ne!(
            block1.hash, block2.hash,
            "Blocks with different properties should have different hashes"
        );
    }
}
