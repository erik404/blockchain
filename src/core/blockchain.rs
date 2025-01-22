use crate::config::Config;
use crate::core::block::*;
use crate::errors::transaction_errors::*;
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
    pub fn new(config: Config) -> Result<Self, String> {
        if config.token.total_supply < config.blockchain.genesis_pre_mined {
            return Err("ERR_TOTAL_SUPPLY_LESS_THAN_PRE_MINED".to_string());
        }

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
            vec![],
            config.blockchain.genesis_hash,
            config.blockchain.difficulty,
        );

        // Return the new Blockchain instance
        Ok(Blockchain {
            chain: vec![genesis_block],
            accounts,
            token,
            mempool: vec![],
            difficulty: config.blockchain.difficulty,
        })
    }

    /// Adds a new block to the blockchain:
    /// - Processes valid transactions from the mempool.
    /// - Creates a new block with these transactions.
    /// - Appends the block to the chain.
    /// - Validates the entire blockchain after adding the block.
    /// - Executes the transactions (updates balances).
    pub fn add_block(&mut self) {
        // Process the mempool and collect valid transactions
        let valid_transactions: Vec<Transaction> = self.process_mempool();

        // Get the last block and create the new index
        let last_block: &Block = self.chain.last().unwrap();
        let new_block_index: u32 = last_block.index + 1;

        // Create a new block with valid transactions
        let new_block: Block = Block::new(
            new_block_index,
            valid_transactions.clone(),
            last_block.hash.clone(),
            self.difficulty,
        );
        // Validate block with the network
        // TODO
        
        // Add the new block to the chain
        self.chain.push(new_block);
        
        // Validate the blockchain after adding the new block
        if !self.is_valid() {
            eprintln!(
                "Blockchain is invalid after adding block {}. Rolling back.",
                new_block_index
            );
            self.chain.pop();
            return;
        }
        
        // Execute transactions
        self.execute_transactions(&valid_transactions);
    }

    /// Loops through the pending transactions and return the valid ones.
    fn process_mempool(&mut self) -> Vec<Transaction> {
        let mut valid_transactions: Vec<Transaction> = Vec::new();
        let mut temp_balances: HashMap<String, u64> = self.accounts.clone();

        for transaction in &self.mempool {
            if let Err(why) =
                self.validate_transaction_with_temp_balances(transaction, &mut temp_balances)
            {
                eprintln!("Transaction validation failed: {}", why);
                continue;
            }
            // Add transaction to the list of valid transactions
            valid_transactions.push(transaction.clone());
        }

        // Clear the mempool after processing
        self.mempool.clear(); // todo not sure if clearing the mempool here is the right spot
        valid_transactions
    }
    
    fn execute_transactions(&mut self, valid_transactions: &Vec<Transaction>) {
        // Execute the transactions and update account balances
        for transaction in valid_transactions {
            // Deduct the transaction amount from the sender's balance
            *self.accounts.entry(transaction.sender.clone()).or_insert(0) -= transaction.amount;
            // Add the transaction amount to the receiver's balance
            *self
                .accounts
                .entry(transaction.receiver.clone())
                .or_insert(0) += transaction.amount;
        }
    }



    fn validate_transaction_with_temp_balances(
        &self,
        transaction: &Transaction,
        temp_balances: &mut HashMap<String, u64>,
    ) -> Result<(), TransactionError> {
        // Ensure sender and receiver addresses are valid
        if transaction.sender.is_empty() || transaction.receiver.is_empty() {
            return Err(TransactionError::AddressesCannotBeEmpty);
        }
        // Ensure that the sender is not sending to itself
        if transaction.sender == transaction.receiver {
            return Err(TransactionError::SenderAndReceiverCannotBeTheSame);
        }
        // Ensure the transaction amount is greater than zero
        if transaction.amount == 0 {
            return Err(TransactionError::AmountMustBeGreaterThanZero);
        }
        // Check if the sender has enough tokens in the temporary balance map
        if let Some(balance) = temp_balances.get_mut(&transaction.sender) {
            if *balance >= transaction.amount {
                // Deduct the transaction amount from the temporary balance
                *balance -= transaction.amount;

                // Add the transaction amount to the receiver's temporary balance
                *temp_balances
                    .entry(transaction.receiver.clone())
                    .or_insert(0) += transaction.amount;
            } else {
                return Err(TransactionError::InsufficientBalance {
                    sender: transaction.sender.clone(),
                    requested: transaction.amount,
                    available: *balance,
                });
            }
        } else {
            return Err(TransactionError::SenderDoesNotExist {
                sender: transaction.sender.clone(),
            });
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
#[path = "../tests/core/blockchain_test.rs"]
mod blockchain_test;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::config::{BlockchainConfig, TokenConfig};
//     use std::collections::HashMap;
//
//     // Constants for mock configuration
//     const TOKEN_NAME: &str = "test_name";
//     const TOKEN_SYMBOL: &str = "test_symbol";
//     const DECIMALS: u8 = 10;
//     const TOTAL_SUPPLY: u64 = 200_000_000_000;
//     const GENESIS_NAME: &str = "genesis_name";
//     const DIFFICULTY: usize = 2;
//     const GENESIS_PRE_MINED: u64 = 2_000_000_000_000_000_000;
//     const GENESIS_MINER: &str = "Miner";
//
//     // Helper function to create a mock configuration for testing
//     fn mock_config() -> Config {
//         Config {
//             token: TokenConfig {
//                 name: TOKEN_NAME.to_string(),
//                 symbol: TOKEN_SYMBOL.to_string(),
//                 decimals: DECIMALS,
//                 total_supply: TOTAL_SUPPLY,
//             },
//             blockchain: BlockchainConfig {
//                 genesis_name: GENESIS_NAME.to_string(),
//                 difficulty: DIFFICULTY,
//                 genesis_pre_mined: GENESIS_PRE_MINED,
//                 genesis_miner: GENESIS_MINER.to_string(),
//             },
//         }
//     }
//
//     // Helper function to create a transaction
//     fn create_transaction(sender: &str, receiver: &str, amount: u64) -> Transaction {
//         Transaction::new(sender.to_string(), receiver.to_string(), amount)
//     }
//
//     #[test]
//     fn test_blockchain_initialization() {
//         let config: Config = mock_config(); // Use the mock configuration helper
//                                             // Initialize the blockchain
//         let blockchain: Blockchain = Blockchain::new(config.clone());
//         // Validate the token configuration
//         assert_eq!(
//             blockchain.token.name, config.token.name,
//             "Token name does not match the configuration"
//         );
//         assert_eq!(
//             blockchain.token.symbol, config.token.symbol,
//             "Token symbol does not match the configuration"
//         );
//         assert_eq!(
//             blockchain.token.decimals, config.token.decimals,
//             "Token decimals do not match the configuration"
//         );
//         assert_eq!(
//             blockchain.token.total_supply, config.token.total_supply,
//             "Token total supply does not match the configuration"
//         );
//         // Validate the genesis account
//         assert_eq!(
//             blockchain.accounts.get(&config.blockchain.genesis_miner),
//             Some(&config.blockchain.genesis_pre_mined),
//             "Genesis miner account was not initialized with the correct balance"
//         );
//         // Validate the genesis block
//         let genesis_block: &Block = &blockchain.chain[0];
//         assert_eq!(genesis_block.index, 0, "Genesis block index should be 0");
//         assert_eq!(
//             genesis_block.previous_hash, config.blockchain.genesis_name,
//             "Genesis block previous_hash does not match the configuration"
//         );
//         assert_eq!(
//             genesis_block.transactions.len(),
//             1,
//             "Genesis block should contain one dummy transaction"
//         );
//         assert_eq!(
//             genesis_block.transactions[0],
//             Transaction::new("".to_string(), "".to_string(), 0),
//             "Genesis block's dummy transaction does not match the expected default"
//         );
//         // Validate difficulty
//         assert_eq!(
//             blockchain.difficulty, config.blockchain.difficulty,
//             "Blockchain difficulty does not match the configuration"
//         );
//         // Validate mempool
//         assert!(
//             blockchain.mempool.is_empty(),
//             "Mempool should be empty on initialization"
//         );
//     }
//
//     #[test]
//     fn test_blockchain_is_valid() {
//         let mut blockchain: Blockchain = Blockchain::new(mock_config());
//
//         // Add valid blocks
//         let block1_transactions: Vec<Transaction> = vec![
//             create_transaction("Alice", "Bob", 50),
//             create_transaction("Bob", "Charlie", 25),
//         ];
//         let block1 = Block::new(
//             1,
//             block1_transactions.clone(),
//             blockchain.chain[0].hash.clone(),
//             mock_config().blockchain.difficulty,
//         );
//         blockchain.chain.push(block1);
//
//         let block2_transactions: Vec<Transaction> = vec![
//             create_transaction("Charlie", "Alice", 10),
//             create_transaction("Alice", "David", 5),
//         ];
//         let block2: Block = Block::new(
//             2,
//             block2_transactions.clone(),
//             blockchain.chain[1].hash.clone(),
//             mock_config().blockchain.difficulty,
//         );
//         blockchain.chain.push(block2);
//
//         // Test a valid blockchain
//         assert!(
//             blockchain.is_valid(),
//             "Blockchain with valid blocks should be valid"
//         );
//
//         // Tamper with a block (break hash link)
//         blockchain.chain[1].previous_hash = "tampered_hash".to_string();
//         assert!(
//             !blockchain.is_valid(),
//             "Blockchain with a broken hash link should be invalid"
//         );
//
//         // Restore the link and tamper with a block's hash
//         blockchain.chain[1].previous_hash = blockchain.chain[0].hash.clone();
//         blockchain.chain[1].hash = "tampered_hash".to_string();
//         assert!(
//             !blockchain.is_valid(),
//             "Blockchain with a tampered block hash should be invalid"
//         );
//
//         // Restore the hash and tamper with the block's data
//         blockchain.chain[1].hash = Block::calculate_hash(
//             blockchain.chain[1].index,
//             &blockchain.chain[1].timestamp,
//             &blockchain.chain[1].transactions,
//             &blockchain.chain[1].previous_hash,
//             blockchain.chain[1].nonce,
//         );
//         blockchain.chain[1].transactions[0] = create_transaction("Alice", "Eve", 9999);
//         assert!(
//             !blockchain.is_valid(),
//             "Blockchain with tampered block data should be invalid"
//         );
//     }
//
//     #[test]
//     fn test_blockchain_rollback() {
//         let mut blockchain: Blockchain = Blockchain::new(mock_config());
//
//         // Add a valid block first
//         blockchain.mempool = vec![create_transaction("Alice", "Bob", 30)];
//         blockchain.add_block();
//
//         // Ensure the valid block was added
//         assert_eq!(
//             blockchain.chain.len(),
//             2,
//             "Blockchain should contain 2 blocks after adding a valid block"
//         );
//
//         // Manually append an invalid block (simulate tampering)
//         let tampered_block = Block::new(
//             blockchain.chain.len() as u32,
//             vec![],
//             "invalid_previous_hash".to_string(), // Invalid previous hash
//             blockchain.difficulty,
//         );
//         blockchain.chain.push(tampered_block);
//
//         // Validate the chain and trigger rollback
//         if !blockchain.is_valid() {
//             blockchain.chain.pop(); // Rollback the invalid block
//         }
//         // Ensure the chain length remains consistent after rollback
//         assert_eq!(
//             blockchain.chain.len(),
//             2,
//             "Blockchain length should remain unchanged after rolling back an invalid block"
//         );
//     }
//
//     #[test]
//     fn test_empty_address_sender() {
//         let blockchain: Blockchain = Blockchain::new(mock_config());
//         let mut temp_balances: HashMap<String, u64> = blockchain.accounts.clone();
//         let transaction: Transaction = create_transaction("", "Bob", 10);
//         assert!(
//             blockchain
//                 .validate_transaction_with_temp_balances(&transaction, &mut temp_balances)
//                 .is_err(),
//             "Transaction with empty sender address should be invalid"
//         );
//     }
//
//     #[test]
//     fn test_empty_address_receiver() {
//         let blockchain: Blockchain = Blockchain::new(mock_config());
//         let mut temp_balances: HashMap<String, u64> = blockchain.accounts.clone();
//         let transaction: Transaction = create_transaction("Bob", "", 10);
//         assert!(
//             blockchain
//                 .validate_transaction_with_temp_balances(&transaction, &mut temp_balances)
//                 .is_err(),
//             "Transaction with empty sender address should be invalid"
//         );
//     }
//
//     #[test]
//     fn test_same_sender_and_receiver() {
//         let blockchain: Blockchain = Blockchain::new(mock_config());
//         let mut temp_balances: HashMap<String, u64> = blockchain.accounts.clone();
//         let transaction: Transaction = create_transaction("Alice", "Alice", 10);
//         assert!(
//             blockchain
//                 .validate_transaction_with_temp_balances(&transaction, &mut temp_balances)
//                 .is_err(),
//             "Transaction where sender and receiver are the same should be invalid"
//         );
//     }
//
//     #[test]
//     fn test_zero_amount() {
//         let blockchain: Blockchain = Blockchain::new(mock_config());
//         let mut temp_balances: HashMap<String, u64> = blockchain.accounts.clone();
//         let transaction: Transaction = create_transaction("Alice", "Bob", 0);
//         assert!(
//             blockchain
//                 .validate_transaction_with_temp_balances(&transaction, &mut temp_balances)
//                 .is_err(),
//             "Transaction with zero amount should be invalid"
//         );
//     }
//
//     #[test]
//     fn test_insufficient_balance() {
//         let mut blockchain: Blockchain = Blockchain::new(mock_config());
//         blockchain.accounts.insert("Alice".to_string(), 50);
//         let mut temp_balances: HashMap<String, u64> = blockchain.accounts.clone();
//         let transaction: Transaction = create_transaction("Alice", "Bob", 100);
//         assert!(
//             blockchain
//                 .validate_transaction_with_temp_balances(&transaction, &mut temp_balances)
//                 .is_err(),
//             "Transaction where sender has insufficient balance should be invalid"
//         );
//     }
//
//     #[test]
//     fn test_multiple_transactions_with_insufficient_balance() {
//         let mut blockchain: Blockchain = Blockchain::new(mock_config());
//         blockchain.accounts.insert("Alice".to_string(), 100);
//         let mut temp_balances: HashMap<String, u64> = blockchain.accounts.clone();
//
//         let transactions: Vec<Transaction> = vec![
//             create_transaction("Alice", "Bob", 50),
//             create_transaction("Alice", "Charlie", 50),
//             create_transaction("Alice", "David", 50), // Should fail
//         ];
//
//         let valid_transactions: Vec<_> = transactions
//             .into_iter()
//             .filter(|tx| {
//                 blockchain
//                     .validate_transaction_with_temp_balances(tx, &mut temp_balances)
//                     .is_ok()
//             })
//             .collect();
//
//         assert_eq!(
//             valid_transactions.len(),
//             2,
//             "Only two transactions should be valid due to insufficient balance"
//         );
//     }
//
//     #[test]
//     fn test_valid_transaction() {
//         let mut blockchain: Blockchain = Blockchain::new(mock_config());
//         blockchain.accounts.insert("Alice".to_string(), 100);
//         let mut temp_balances: HashMap<String, u64> = blockchain.accounts.clone();
//         let transaction: Transaction = create_transaction("Alice", "Bob", 50);
//         assert!(
//             blockchain
//                 .validate_transaction_with_temp_balances(&transaction, &mut temp_balances)
//                 .is_ok(),
//             "Transaction with sufficient balance, valid sender/receiver, and non-zero amount should be valid"
//         );
//     }
//
//     #[test]
//     fn test_process_mempool() {
//         let mut blockchain: Blockchain = Blockchain::new(mock_config());
//         blockchain.accounts.insert("Alice".to_string(), 100);
//         blockchain.mempool = vec![
//             create_transaction("Alice", "Bob", 50),
//             create_transaction("Alice", "Charlie", 30),
//             create_transaction("Alice", "David", 50), // Should fail
//         ];
//
//         let valid_transactions = blockchain.process_mempool();
//
//         assert_eq!(
//             valid_transactions.len(),
//             2,
//             "Only two transactions should be valid and included in the processed mempool"
//         );
//         assert!(
//             blockchain.mempool.is_empty(),
//             "Mempool should be cleared after processing"
//         );
//     }
//
//     #[test]
//     fn test_invalid_sender() {
//         let blockchain: Blockchain = Blockchain::new(mock_config());
//         let mut temp_balances: HashMap<String, u64> = blockchain.accounts.clone();
//         let transaction: Transaction = create_transaction("NonExistent", "Bob", 50);
//         assert!(
//             blockchain
//                 .validate_transaction_with_temp_balances(&transaction, &mut temp_balances)
//                 .is_err(),
//             "Transaction with a non-existent sender should be invalid"
//         );
//     }
//
//     #[test]
//     fn test_get_transaction_history() {
//         let mut blockchain: Blockchain = Blockchain::new(mock_config());
//
//         // Add initial balances
//         blockchain.accounts.insert("Alice".to_string(), 100);
//         blockchain.accounts.insert("Bob".to_string(), 50);
//         blockchain.accounts.insert("Charlie".to_string(), 30);
//
//         // Add some blocks with transactions
//         let transactions_block1: Vec<Transaction> = vec![
//             create_transaction("Alice", "Bob", 20),
//             create_transaction("Bob", "Charlie", 10),
//         ];
//         let block1: Block = Block::new(
//             1,
//             transactions_block1.clone(),
//             "genesis_hash".to_string(),
//             2,
//         );
//         blockchain.chain.push(block1);
//
//         let transactions_block2: Vec<Transaction> = vec![
//             create_transaction("Charlie", "Alice", 5),
//             create_transaction("Alice", "Charlie", 15),
//         ];
//         let block2: Block =
//             Block::new(2, transactions_block2.clone(), "block1_hash".to_string(), 2);
//         blockchain.chain.push(block2);
//
//         // Query Alice's transaction history
//         let alice_history: Vec<Transaction> =
//             blockchain.get_transaction_history(&"Alice".to_string());
//
//         // Check that all transactions involving Alice are returned
//         let expected_transactions: Vec<Transaction> = vec![
//             create_transaction("Alice", "Bob", 20),
//             create_transaction("Charlie", "Alice", 5),
//             create_transaction("Alice", "Charlie", 15),
//         ];
//         assert_eq!(
//             alice_history, expected_transactions,
//             "Transaction history for Alice does not match expected history"
//         );
//
//         // Query Bob's transaction history
//         let bob_history: Vec<Transaction> = blockchain.get_transaction_history(&"Bob".to_string());
//         let expected_bob_transactions = vec![
//             create_transaction("Alice", "Bob", 20),
//             create_transaction("Bob", "Charlie", 10),
//         ];
//         assert_eq!(
//             bob_history, expected_bob_transactions,
//             "Transaction history for Bob does not match expected history"
//         );
//
//         // Query an address with no transactions
//         let no_history: Vec<Transaction> = blockchain.get_transaction_history(&"NoOne".to_string());
//         assert!(
//             no_history.is_empty(),
//             "Transaction history for an address with no transactions should be empty"
//         );
//     }
//
//     #[test]
//     fn test_execute_transactions() {
//         let mut blockchain = Blockchain::new(mock_config());
//
//         // Initialize account balances
//         blockchain.accounts.insert("Alice".to_string(), 100);
//         blockchain.accounts.insert("Bob".to_string(), 50);
//
//         // Create valid transactions
//         let valid_transactions = vec![
//             create_transaction("Alice", "Bob", 30), // Alice sends 30 to Bob
//             create_transaction("Bob", "Alice", 20), // Bob sends 20 to Alice
//         ];
//
//         // Execute the transactions
//         blockchain.execute_transactions(&valid_transactions);
//
//         // Check updated balances
//         assert_eq!(
//             blockchain.accounts.get("Alice"),
//             Some(&90),
//             "Alice's balance should be updated correctly after transactions"
//         );
//         assert_eq!(
//             blockchain.accounts.get("Bob"),
//             Some(&60),
//             "Bob's balance should be updated correctly after transactions"
//         );
//     }
//
// }
