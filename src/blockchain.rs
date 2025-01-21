use crate::block::*;

#[derive(Debug)]
pub(crate) struct Blockchain {
    pub(crate) chain: Vec<Block>,
    difficulty: usize, // Difficulty: Number of leading zeros
}

impl Blockchain {
    // Create a new blockchain with a genesis block
    pub(crate) fn new(difficulty: usize) -> Self {
        let genesis_block = Block::new(0, vec!["Genesis Block".to_string()], "0".to_string(), difficulty);
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    // Add a new block to the blockchain
    pub(crate) fn add_block(&mut self, transactions: Vec<String>) {
        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            transactions,
            last_block.hash.clone(),
            self.difficulty
        );
        self.chain.push(new_block);
    }

    // Verify the integrity of the blockchain
    pub(crate) fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Check if the hashes match
            if current_block.previous_hash != previous_block.hash {
                println!("Chain is broken at block {}!", current_block.index);
                return false;
            }

            // Check if the current block's hash is valid
            let recalculated_hash = Block::calculate_hash(
                current_block.index,
                &current_block.timestamp,
                &current_block.transactions,
                &current_block.previous_hash,
                current_block.nonce
            );
            if current_block.hash != recalculated_hash {
                println!("Block {} has invalid hash!", current_block.index);
                return false;
            }
        }
        true
    }
}
