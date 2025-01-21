use crate::config::Config;
use crate::core::block::*;
use crate::structs::token::Token;
use crate::structs::transaction::Transaction;
use std::collections::HashMap;

/// Represents the blockchain structure.
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,              // Sequence of blocks forming the blockchain
    pub token: Token,                   // Information about the blockchain's native token
    pub mempool: Vec<Transaction>,      // Pool of pending transactions
    pub accounts: HashMap<String, u64>, // Map of account addresses to balances
    difficulty: usize,                  // Mining difficulty level
}

impl Blockchain {
    /// Creates a new Blockchain instance.
    /// - Initializes the token based on the configuration.
    /// - Sets up accounts with a pre-mined balance.
    /// - Adds a genesis block to start the chain.
    pub fn new(config: Config) -> Self {
        // Initialize the token based on the provided configuration
        let token: Token = Token::new(
            config.token.name.clone(),
            config.token.symbol.clone(),
            config.token.decimals,
            config.token.total_supply,
        );

        // Initialize accounts with a pre-mined balance
        let mut accounts: HashMap<String, u64> = HashMap::new();
        accounts.insert(
            config.blockchain.genesis_miner,
            config.blockchain.genesis_pre_mined,
        );

        // Create the genesis block
        let genesis_block = Block::new(
            0,
            vec![Transaction::new("".to_string(), "".to_string(), 0)],
            config.blockchain.genesis_name,
            config.blockchain.difficulty,
        );

        // Return the new Blockchain instance
        Blockchain {
            chain: vec![genesis_block],
            accounts,
            token,
            mempool: vec![],
            difficulty: config.blockchain.difficulty,
        }
    }

    /// Adds a new block to the blockchain:
    /// - Processes valid transactions from the mempool.
    /// - Creates a new block with these transactions.
    /// - Appends the block to the chain.
    /// - Validates the entire blockchain after adding the block.
    pub fn add_block(&mut self) {
        // Process the mempool and collect valid transactions
        let processed_transactions: Vec<Transaction> = self.process_mempool(); // TODO: Consider error handling for invalid transactions
                                                                               // Get the last block in the chain
        let last_block: &Block = self.chain.last().unwrap();

        // Create a new block with valid transactions
        let new_block: Block = Block::new(
            last_block.index + 1,
            processed_transactions,
            last_block.hash.clone(),
            self.difficulty,
        );

        // Add the new block to the blockchain
        self.chain.push(new_block);
        // Validate the blockchain after adding the new block
        self.is_valid();
    }

    /// Processes all transactions in the mempool:
    /// - Valid transactions are executed, updating account balances.
    /// - Invalid transactions are logged and skipped.
    ///   
    /// Returns a vector of successfully processed transactions.
    fn process_mempool(&mut self) -> Vec<Transaction> {
        let mut processed_transactions: Vec<Transaction> = vec![];

        for transaction in &self.mempool {
            match self.validate_transaction(transaction) {
                Ok(_) => {
                    // Deduct the transaction amount from the sender's balance
                    *self.accounts.entry(transaction.sender.clone()).or_insert(0) -=
                        transaction.amount;

                    // Add the transaction amount to the receiver's balance
                    *self
                        .accounts
                        .entry(transaction.receiver.clone())
                        .or_insert(0) += transaction.amount;

                    // Add the transaction to the list of processed transactions
                    processed_transactions.push(transaction.clone());
                }
                Err(why) => {
                    // Log the rejection reason for invalid transactions
                    eprintln!(
                        "Rejected transaction: {} -> {}: {} (Reason: {})",
                        transaction.sender, transaction.receiver, transaction.amount, why
                    );
                }
            }
        }

        // Clear the mempool after processing
        self.mempool.clear();
        // Return the processed transactions
        processed_transactions
    }

    /// Validates a transaction for correctness and sufficient balance.
    /// Returns `Ok(())` if valid, otherwise returns an error message.
    fn validate_transaction(&self, transaction: &Transaction) -> Result<(), String> {
        // Ensure sender and receiver addresses are valid
        if transaction.sender.is_empty() || transaction.receiver.is_empty() {
            return Err("Transaction addresses cannot be empty.".to_string());
        }

        // Ensure that the sender is not sending to itself
        if transaction.sender == transaction.receiver {
            return Err("Sender and receiver cannot be the same.".to_string());
        }

        // Ensure the transaction amount is greater than zero
        if transaction.amount == 0 {
            return Err("Transaction amount must be greater than zero.".to_string());
        }

        // Check if the sender has sufficient balance
        if let Some(balance) = self.accounts.get(&transaction.sender) {
            if *balance < transaction.amount {
                return Err(format!(
                    "Insufficient balance: {} has {} but tried to send {}",
                    transaction.sender, balance, transaction.amount
                ));
            }
        } else {
            return Err(format!("Sender {} does not exist.", transaction.sender));
        }

        Ok(())
    }

    /// Validates the blockchain integrity.
    /// Ensures hashes match and blocks are correctly linked.
    pub fn is_valid(&self) -> bool {
        // Loop starts at 1: Skips the genesis block, as it has no previous block
        for i in 1..self.chain.len() {
            let current_block: &Block = &self.chain[i];
            let previous_block: &Block = &self.chain[i - 1];

            // Verify the previous hash matches the hash of the preceding block
            if current_block.previous_hash != previous_block.hash {
                eprintln!("Chain is broken at block {}!", current_block.index);
                return false;
            }

            // Recalculate the current block's hash and validate it
            let recalculated_hash: String = Block::calculate_hash(
                current_block.index,
                &current_block.timestamp,
                &current_block.transactions,
                &current_block.previous_hash,
                current_block.nonce,
            );
            if current_block.hash != recalculated_hash {
                eprintln!("Block {} has an invalid hash!", current_block.index);
                return false;
            }
        }

        true
    }

    /// Returns the transaction history for the given address.
    /// Scans all blocks in the chain and filters transactions involving the address.
    pub fn get_transaction_history(&self, address: &String) -> Vec<Transaction> {
        self.chain
            .iter() // Iterate over all blocks in the blockchain
            .flat_map(|block| {
                block
                    .transactions // Access transactions in the block
                    .iter() // Iterate over transactions
                    .filter(|tx| tx.sender == *address || tx.receiver == *address) // Filter transactions involving the address
                    .cloned() // Clone transactions to avoid borrowing issues
            })
            .collect() // Collect matching transactions into a Vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{BlockchainConfig, TokenConfig};

    // Constants for mock configuration
    const TOKEN_NAME: &str = "test_name";
    const TOKEN_SYMBOL: &str = "test_symbol";
    const DECIMALS: u8 = 10;
    const TOTAL_SUPPLY: u64 = 200_000_000_000;
    const GENESIS_NAME: &str = "genesis_name";
    const DIFFICULTY: usize = 2;
    const GENESIS_PRE_MINED: u64 = 2000000000000000000;
    const GENESIS_MINER: &str = "Miner";

    // Helper function to create a mock configuration for testing
    fn mock_config() -> Config {
        Config {
            token: TokenConfig {
                name: TOKEN_NAME.to_string(),
                symbol: TOKEN_SYMBOL.to_string(),
                decimals: DECIMALS,
                total_supply: TOTAL_SUPPLY,
            },
            blockchain: BlockchainConfig {
                genesis_name: GENESIS_NAME.to_string(),
                difficulty: DIFFICULTY,
                genesis_pre_mined: GENESIS_PRE_MINED,
                genesis_miner: GENESIS_MINER.to_string(),
            },
        }
    }

    // Helper function to create a transaction
    fn create_transaction(sender: &str, receiver: &str, amount: u64) -> Transaction {
        Transaction::new(sender.to_string(), receiver.to_string(), amount)
    }

    #[test]
    fn test_empty_addresses() {
        let blockchain: Blockchain = Blockchain::new(mock_config());
        let transaction: Transaction = create_transaction("", "Bob", 10);
        assert!(
            blockchain.validate_transaction(&transaction).is_err(),
            "Transaction with empty sender address should be invalid"
        );
    }

    #[test]
    fn test_same_sender_and_receiver() {
        let blockchain: Blockchain = Blockchain::new(mock_config());
        let transaction: Transaction = create_transaction("Alice", "Alice", 10);
        assert!(
            blockchain.validate_transaction(&transaction).is_err(),
            "Transaction where sender and receiver are the same should be invalid"
        );
    }

    #[test]
    fn test_zero_amount() {
        let blockchain: Blockchain = Blockchain::new(mock_config());
        let transaction: Transaction = create_transaction("Alice", "Bob", 0);
        assert!(
            blockchain.validate_transaction(&transaction).is_err(),
            "Transaction with zero amount should be invalid"
        );
    }

    #[test]
    fn test_insufficient_balance() {
        let mut blockchain: Blockchain = Blockchain::new(mock_config());
        blockchain.accounts.insert("Alice".to_string(), 50);
        let transaction: Transaction = create_transaction("Alice", "Bob", 100);
        assert!(
            blockchain.validate_transaction(&transaction).is_err(),
            "Transaction where sender has insufficient balance should be invalid"
        );
    }

    #[test]
    fn test_valid_transaction() {
        let mut blockchain: Blockchain = Blockchain::new(mock_config());
        blockchain.accounts.insert("Alice".to_string(), 100);
        let transaction: Transaction = create_transaction("Alice", "Bob", 50);
        assert!(
            blockchain.validate_transaction(&transaction).is_ok(),
            "Transaction with sufficient balance, valid sender/receiver, and non-zero amount should be valid"
        );
    }
}
