use crate::common::calculate_hash::calculate_block_hash;
use crate::config::Config;
use crate::core::block::*;
use crate::errors::transaction_errors::*;
use crate::structs::token::Token;
use crate::structs::transaction::Transaction;
use std::collections::HashMap;

/// Represents the blockchain structure.
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub token: Token,
    pub mempool: Vec<Transaction>,
    pub accounts: HashMap<String, u64>,
    difficulty: usize,
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
        let token = Token::new(
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

        if valid_transactions.is_empty() {
            return; // Never create a new block when there are no transactions
        }

        let last_block = self.chain.last().unwrap();
        let new_block_index = last_block.index + 1;

        let new_block = Block::new(
            new_block_index,
            valid_transactions.clone(),
            last_block.hash.clone(),
            self.difficulty,
        );

        // Validate block with the network
        // TODO

        self.chain.push(new_block);

        if !self.is_valid() {
            eprintln!(
                "Blockchain is invalid after adding block {}. Rolling back.",
                new_block_index
            );
            self.chain.pop();
            return;
        }

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
            valid_transactions.push(transaction.clone());
        }

        self.mempool.clear(); // todo not sure if clearing the mempool here is the right spot
        valid_transactions
    }

    /// Executes a list of valid transactions and updates the account balances accordingly.
    ///
    /// This function iterates over the provided list of valid transactions and:
    /// - Deducts the transaction amount from the sender's balance.
    /// - Adds the transaction amount to the receiver's balance.
    ///
    /// It assumes that all transactions in the provided list are already validated and
    /// no further validation is performed.
    fn execute_transactions(&mut self, valid_transactions: &Vec<Transaction>) {
        for transaction in valid_transactions {
            *self.accounts.entry(transaction.sender.clone()).or_insert(0) -= transaction.amount;
            *self
                .accounts
                .entry(transaction.receiver.clone())
                .or_insert(0) += transaction.amount;
        }
    }

    /// Validates a transaction and updates temporary balances if the transaction is valid.
    ///
    /// This function performs the following checks:
    /// - Ensures that the sender and receiver addresses are not empty.
    /// - Ensures that the sender and receiver are not the same address.
    /// - Ensures that the transaction amount is greater than zero.
    /// - Ensures that the sender has sufficient balance in the provided temporary balances.
    ///
    /// If all validations pass, the transaction amount is deducted from the sender's balance
    /// and added to the receiver's balance in the provided `temp_balances` map.
    ///
    fn validate_transaction_with_temp_balances(
        &self,
        transaction: &Transaction,
        temp_balances: &mut HashMap<String, u64>,
    ) -> Result<(), TransactionError> {
        if transaction.sender.is_empty() || transaction.receiver.is_empty() {
            return Err(TransactionError::AddressCannotBeEmpty);
        }
        if transaction.sender == transaction.receiver {
            return Err(TransactionError::SenderAndReceiverCannotBeTheSame);
        }
        if transaction.amount == 0 {
            return Err(TransactionError::AmountMustBeGreaterThanZero);
        }
        let receiver_balance = *temp_balances.entry(transaction.receiver.clone()).or_insert(0);
        if receiver_balance.checked_add(transaction.amount).is_none() {
            return Err(TransactionError::BalanceOverflow);
        }
        let sender_balance: &mut u64 = temp_balances.get_mut(&transaction.sender).ok_or(
            TransactionError::SenderDoesNotExist {
                sender: transaction.sender.clone(),
            },
        )?;
        if *sender_balance < transaction.amount {
            return Err(TransactionError::InsufficientBalance {
                sender: transaction.sender.clone(),
                requested: transaction.amount,
                available: *sender_balance,
            });
        }

        *sender_balance -= transaction.amount;
        *temp_balances
            .entry(transaction.receiver.clone())
            .or_insert(0) += transaction.amount;

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
            let recalculated_hash: String = calculate_block_hash(
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
            .iter()
            .flat_map(|block| {
                block
                    .transactions
                    .iter()
                    .filter(|tx| tx.sender == *address || tx.receiver == *address)
                    .cloned()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::calculate_hash::calculate_block_hash;
    use crate::test_utils::mock_config;
    #[test]
    fn validate_genesis_block() {
        // Initialize the blockchain using the mock configuration
        let config = mock_config();
        let blockchain = Blockchain::new(config.clone()).unwrap();

        let block = blockchain.chain.last().unwrap();

        assert_eq!(block.index, 0, "Genesis index should be 0");

        assert_eq!(
            block.previous_hash, config.blockchain.genesis_hash,
            "Genesis block hash should be equal to config.blockchain.genesis_hash"
        );

        assert_eq!(block.transactions, vec![], "Transactions should be empty");
    }
    #[test]
    fn validate_initial_blockchain_state() {
        // Initialize the blockchain using the mock configuration
        let config = mock_config();
        let blockchain = Blockchain::new(config.clone()).unwrap();

        // Check that the mempool is empty at initialization
        assert_eq!(blockchain.mempool, vec![], "Mempool should be empty");

        // Check that the difficulty matches the configuration
        assert_eq!(
            blockchain.difficulty, config.blockchain.difficulty,
            "Difficulty should be equal to config.blockchain.difficulty"
        );

        // Verify that the accounts are initialized with the genesis miner and pre-mined tokens
        let mut accounts: HashMap<String, u64> = HashMap::new();
        accounts.insert(
            config.blockchain.genesis_miner,
            config.blockchain.genesis_pre_mined,
        );
        assert_eq!(
            blockchain.accounts, accounts,
            "Accounts should hold genesis_miner and genesis_pre_mined"
        );

        // Verify that the token is initialized correctly based on the configuration
        let token: Token = Token::new(
            config.token.name,
            config.token.symbol,
            config.token.decimals,
            config.token.total_supply,
        );
        assert_eq!(blockchain.token, token, "Token should be equal");

        // Check that the blockchain starts with only the genesis block
        assert_eq!(
            blockchain.chain.len(),
            1,
            "Blockchain should start with 1 block (genesis)"
        );

        // Ensure the blockchain is valid upon initialization
        assert!(blockchain.is_valid(), "Blockchain should be valid");
    }
    #[test]
    fn validate_token_initialization() {
        // Initialize the blockchain using the mock configuration
        let config = mock_config();
        let blockchain: Blockchain = Blockchain::new(config.clone()).unwrap();

        assert_eq!(
            blockchain.token.name, config.token.name,
            "Token name does not match the configuration"
        );

        assert_eq!(
            blockchain.token.total_supply, config.token.total_supply,
            "Token total_supply does not match the configuration"
        );

        assert_eq!(
            blockchain.token.symbol, config.token.symbol,
            "Token symbol does not match the configuration"
        );

        assert_eq!(
            blockchain.token.decimals, config.token.decimals,
            "Token decimals do not match the configuration"
        );

        assert_eq!(
            blockchain.token.smallest_unit,
            10u64.pow(blockchain.token.decimals as u32),
            "Smallest unit calculation does not match the configuration"
        );
    }
    #[test]
    fn pre_mined_cannot_exceed_total_supply() {
        // Create a mock configuration for the blockchain
        let mut config = mock_config();

        // Set the pre-mined amount to a value greater than the total token supply
        config.blockchain.genesis_pre_mined = config.token.total_supply + 1;

        // Attempt to initialize the blockchain with the invalid configuration
        let blockchain_result: Result<Blockchain, String> = Blockchain::new(config.clone());

        // Assert that the initialization fails with the expected error
        assert_eq!(
            blockchain_result.unwrap_err(),
            "ERR_TOTAL_SUPPLY_LESS_THAN_PRE_MINED",
            "The blockchain should return an error if the pre-mined tokens exceed the total supply"
        );
    }
    #[test]
    fn validate_block_addition_to_blockchain() {
        // Initialize the blockchain with a mock configuration
        let config = mock_config();
        let mut blockchain = Blockchain::new(config.clone()).unwrap();

        // Assert that the blockchain starts with only the genesis block
        assert_eq!(
            blockchain.chain.len(),
            1,
            "Blockchain should start with the genesis block"
        );

        // Add an account with a balance for testing
        blockchain.accounts.insert("test_a".parse().unwrap(), 100);

        // Add a transaction to the mempool
        blockchain.mempool.push(Transaction::new(
            "test_a".parse().unwrap(),
            "test_b".parse().unwrap(),
            100,
        ));

        // Add a new block to the blockchain
        blockchain.add_block();

        // Get the last block in the chain for validation
        let block = blockchain.chain.last().unwrap();

        // Assert that the blockchain now contains two blocks (genesis + the new block)
        assert_eq!(blockchain.chain.len(), 2, "Blockchain should have 2 blocks");

        // Assert that the last block contains exactly one transaction
        assert_eq!(
            block.transactions.len(),
            1,
            "The last block should contain one transaction"
        );

        // Assert that the transaction in the block matches the one added to the mempool
        assert_eq!(
            block.transactions,
            vec![Transaction::new(
                "test_a".parse().unwrap(),
                "test_b".parse().unwrap(),
                100
            )],
            "Transactions should be added to the last block"
        );

        // Assert that the block's hash is correctly calculated
        assert_eq!(
            block.hash,
            calculate_block_hash(
                1,
                &block.timestamp,
                &block.transactions,
                &block.previous_hash,
                block.nonce,
            ),
            "Hash of the last block should match the calculated hash"
        );

        // Assert that the blockchain is still valid after adding the block
        assert!(
            blockchain.is_valid(),
            "Blockchain should be valid after adding a block"
        );

        // Assert that the mempool is empty after transactions are added to the new block
        assert_eq!(
            blockchain.mempool,
            vec![],
            "Mempool should be empty after adding a block"
        );
    }
    #[test]
    fn tampered_block_invalidates_chain() {
        // Initialize the blockchain with a mock configuration
        let config = mock_config();
        let mut blockchain = Blockchain::new(config.clone()).unwrap();

        // Add a valid block
        blockchain.accounts.insert("test_a".parse().unwrap(), 100);
        blockchain.mempool.push(Transaction::new(
            "test_a".parse().unwrap(),
            "test_b".parse().unwrap(),
            50,
        ));
        blockchain.add_block();

        // Temper the second block
        let block = blockchain.chain.last_mut().unwrap();
        block.timestamp = "2025-01-01T00:00:00Z".to_string();

        assert!(
            !blockchain.is_valid(),
            "The blockchain should be invalid when a block is tampered with"
        );
    }
    #[test]
    fn previous_hash_mismatch_invalidates_chain() {
        // Initialize the blockchain with a mock configuration
        let config = mock_config();
        let mut blockchain = Blockchain::new(config.clone()).unwrap();

        // Add a valid block
        blockchain.accounts.insert("test_a".parse().unwrap(), 100);
        blockchain.mempool.push(Transaction::new(
            "test_a".parse().unwrap(),
            "test_b".parse().unwrap(),
            50,
        ));
        blockchain.add_block();

        // Change the previous_hash
        let block = blockchain.chain.last_mut().unwrap();
        block.previous_hash = "123456789".to_string();

        assert!(
            !blockchain.is_valid(),
            "The blockchain should be invalid when previous hash does not match"
        );
    }
    #[test]
    fn blockchain_rollback_on_invalid_block() {
        // Initialize the blockchain with a mock configuration
        let config = mock_config();
        let mut blockchain = Blockchain::new(config.clone()).unwrap();

        // Add a valid block
        blockchain.accounts.insert("test_a".to_string(), 100);
        blockchain.mempool.push(Transaction::new(
            "test_a".to_string(),
            "test_b".to_string(),
            50,
        ));
        blockchain.add_block();

        // Tamper with the blockchain to make it invalid
        let last_block = blockchain.chain.last_mut().unwrap();
        last_block.previous_hash = "invalid_previous_hash".to_string();

        // Save the chain length before attempting to add the block
        let chain_length_before = blockchain.chain.len();

        // Add another block (this should trigger a rollback)
        blockchain.mempool.push(Transaction::new(
            "test_a".to_string(),
            "test_b".to_string(),
            50,
        ));
        blockchain.add_block();

        // Assert: Verify the chain length has not increased
        assert_eq!(
            blockchain.chain.len(),
            chain_length_before,
            "Blockchain length should not increase after adding an invalid block"
        );

        // Assert: Ensure the blockchain remains invalid
        assert!(
            !blockchain.is_valid(),
            "Blockchain should still be invalid due to tampered block"
        );
    }
    #[test]
    fn reject_transaction_with_empty_addresses() {
        // Initialize the blockchain with a mock configuration
        let config = mock_config();
        let blockchain = Blockchain::new(config.clone()).unwrap();

        let mut temp_balances = HashMap::new();
        temp_balances.insert("Alice".to_string(), 100);

        // Sender is empty
        let transaction = Transaction::new("".to_string(), "Bob".to_string(), 50);
        let result = Blockchain::validate_transaction_with_temp_balances(
            &blockchain,
            &transaction,
            &mut temp_balances,
        );
        assert_eq!(
            result,
            Err(TransactionError::AddressCannotBeEmpty),
            "Transaction with empty sender should fail"
        );

        // Receiver is empty
        let transaction = Transaction::new("Alice".to_string(), "".to_string(), 50);
        let result = Blockchain::validate_transaction_with_temp_balances(
            &blockchain,
            &transaction,
            &mut temp_balances,
        );
        assert_eq!(
            result,
            Err(TransactionError::AddressCannotBeEmpty),
            "Transaction with empty receiver should fail"
        );
    }
    #[test]
    fn reject_transaction_with_same_sender_and_receiver() {
        // Initialize the blockchain with a mock configuration
        let config = mock_config();
        let blockchain = Blockchain::new(config.clone()).unwrap();

        let mut temp_balances = HashMap::new();
        temp_balances.insert("Alice".to_string(), 100);

        // Sender and receiver are the same
        let transaction = Transaction::new("Alice".to_string(), "Alice".to_string(), 50);
        let result = Blockchain::validate_transaction_with_temp_balances(
            &blockchain,
            &transaction,
            &mut temp_balances,
        );
        assert_eq!(
            result,
            Err(TransactionError::SenderAndReceiverCannotBeTheSame),
            "Transaction with same sender and receiver should fail"
        );
    }
    #[test]
    fn reject_transaction_with_zero_amount() {
        // Initialize the blockchain with a mock configuration
        let config = mock_config();
        let blockchain = Blockchain::new(config.clone()).unwrap();

        let mut temp_balances = HashMap::new();
        temp_balances.insert("Alice".to_string(), 100);

        // Amount is zero
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 0);
        let result = Blockchain::validate_transaction_with_temp_balances(
            &blockchain,
            &transaction,
            &mut temp_balances,
        );
        assert_eq!(
            result,
            Err(TransactionError::AmountMustBeGreaterThanZero),
            "Transaction with zero amount should fail"
        );
    }
    #[test]
    fn reject_transaction_with_insufficient_balance() {
        // Initialize the blockchain with a mock configuration
        let config = mock_config();
        let blockchain = Blockchain::new(config.clone()).unwrap();

        let mut temp_balances = HashMap::new();
        temp_balances.insert("Alice".to_string(), 50);

        // Amount exceeds sender's balance
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 100);
        let result = Blockchain::validate_transaction_with_temp_balances(
            &blockchain,
            &transaction,
            &mut temp_balances,
        );
        assert_eq!(
            result,
            Err(TransactionError::InsufficientBalance {
                sender: "Alice".to_string(),
                requested: 100,
                available: 50,
            }),
            "Transaction with insufficient balance should fail"
        );
    }
    #[test]
    fn reject_transaction_with_unknown_sender() {
        // Initialize the blockchain with a mock configuration
        let config = mock_config();
        let blockchain = Blockchain::new(config.clone()).unwrap();

        let mut temp_balances = HashMap::new();

        // Sender does not exist in temp_balances
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 50);
        let result = Blockchain::validate_transaction_with_temp_balances(
            &blockchain,
            &transaction,
            &mut temp_balances,
        );
        assert_eq!(
            result,
            Err(TransactionError::SenderDoesNotExist {
                sender: "Alice".to_string(),
            }),
            "Transaction with sender not in temp_balances should fail"
        );
    }
    #[test]
    fn validate_successful_transaction() {
        // Initialize the blockchain with a mock configuration
        let config = mock_config();
        let blockchain = Blockchain::new(config.clone()).unwrap();

        let mut temp_balances = HashMap::new();
        temp_balances.insert("Alice".to_string(), 100);
        temp_balances.insert("Bob".to_string(), 0);

        // Valid transaction
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 50);
        let result = Blockchain::validate_transaction_with_temp_balances(
            &blockchain,
            &transaction,
            &mut temp_balances,
        );
        assert!(result.is_ok(), "Valid transaction should succeed");

        // Check updated balances
        assert_eq!(
            temp_balances["Alice"], 50,
            "Sender's balance should be updated"
        );
        assert_eq!(
            temp_balances["Bob"], 50,
            "Receiver's balance should be updated"
        );
    }
    #[test]
    fn process_mempool_filters_invalid_transactions() {
        // Initialize the blockchain with a mock configuration
        let config = mock_config();
        let mut blockchain = Blockchain::new(config.clone()).unwrap();

        // Add some initial accounts and balances
        blockchain.accounts.insert("Alice".to_string(), 100);
        blockchain.accounts.insert("Bob".to_string(), 50);

        // Add valid and invalid transactions to the mempool
        blockchain
            .mempool
            .push(Transaction::new("Alice".to_string(), "Bob".to_string(), 50)); // Valid
        blockchain.mempool.push(Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            200,
        )); // Invalid: Insufficient balance
        blockchain.mempool.push(Transaction::new(
            "Alice".to_string(),
            "Alice".to_string(),
            50,
        )); // Invalid: Sender and receiver are the same
        blockchain.mempool.push(Transaction::new(
            "Unknown".to_string(),
            "Bob".to_string(),
            50,
        )); // Invalid: Sender does not exist

        // Act: Process the mempool
        let valid_transactions = blockchain.process_mempool();

        // Assert: Only valid transactions should be processed
        assert_eq!(
            valid_transactions.len(),
            1,
            "Only one valid transaction should be processed"
        );
        assert_eq!(
            valid_transactions[0],
            Transaction::new("Alice".to_string(), "Bob".to_string(), 50),
            "The valid transaction should match the expected transaction"
        );

        // Assert: The mempool should be cleared
        assert_eq!(
            blockchain.mempool.len(),
            0,
            "The mempool should be cleared after processing"
        );
    }
    #[test]
    fn retrieve_transaction_history() {
        // Initialize the blockchain using the mock configuration
        let config = mock_config();
        let mut blockchain = Blockchain::new(config.clone()).unwrap();

        // Add accounts and balances
        blockchain.accounts.insert("Alice".to_string(), 200);
        blockchain.accounts.insert("Bob".to_string(), 100);
        blockchain.accounts.insert("Charlie".to_string(), 300);

        // Add a few transactions
        blockchain
            .mempool
            .push(Transaction::new("Alice".to_string(), "Bob".to_string(), 50));
        blockchain.add_block(); // Block 1

        blockchain.mempool.push(Transaction::new(
            "Bob".to_string(),
            "Charlie".to_string(),
            30,
        ));
        blockchain.mempool.push(Transaction::new(
            "Alice".to_string(),
            "Charlie".to_string(),
            70,
        ));
        blockchain.add_block(); // Block 2

        blockchain.mempool.push(Transaction::new(
            "Charlie".to_string(),
            "Alice".to_string(),
            20,
        ));
        blockchain.add_block(); // Block 3

        // Get the transaction history for each address
        let alice_history = blockchain.get_transaction_history(&"Alice".to_string());
        let bob_history = blockchain.get_transaction_history(&"Bob".to_string());
        let charlie_history = blockchain.get_transaction_history(&"Charlie".to_string());

        // Verify Alice's transaction history
        assert_eq!(
            alice_history,
            vec![
                Transaction::new("Alice".to_string(), "Bob".to_string(), 50),
                Transaction::new("Alice".to_string(), "Charlie".to_string(), 70),
                Transaction::new("Charlie".to_string(), "Alice".to_string(), 20)
            ],
            "Alice's transaction history should include all transactions involving her as sender or receiver"
        );

        // Verify Bob's transaction history
        assert_eq!(
            bob_history,
            vec![
                Transaction::new("Alice".to_string(), "Bob".to_string(), 50),
                Transaction::new("Bob".to_string(), "Charlie".to_string(), 30)
            ],
            "Bob's transaction history should include all transactions involving him as sender or receiver"
        );

        // Verify Charlie's transaction history
        assert_eq!(
            charlie_history,
            vec![
                Transaction::new("Bob".to_string(), "Charlie".to_string(), 30),
                Transaction::new("Alice".to_string(), "Charlie".to_string(), 70),
                Transaction::new("Charlie".to_string(), "Alice".to_string(), 20)
            ],
            "Charlie's transaction history should include all transactions involving him as sender or receiver"
        );
    }
    #[test]
    fn block_is_not_created_without_transactions() {
        // Initialize the blockchain using the mock configuration
        let config = mock_config();
        let mut blockchain = Blockchain::new(config.clone()).unwrap();

        assert_eq!(
            blockchain.chain.len(),
            1,
            "Blockchain should start with the genesis block"
        );

        blockchain.add_block();

        assert_eq!(
            blockchain.chain.len(),
            1,
            "No new block should be created when there are no transactions"
        );
    }
    #[test]
    fn add_multiple_blocks_and_verify_chain_integrity_and_balances() {
        // Initialize the blockchain using the mock configuration
        let config = mock_config();
        let mut blockchain = Blockchain::new(config.clone()).unwrap();

        // Set initial balances
        blockchain.accounts.insert("Alice".to_string(), 300);
        blockchain.accounts.insert("Bob".to_string(), 0);
        blockchain.accounts.insert("Charlie".to_string(), 0);

        // Add transactions to the mempool and add the first block
        blockchain.mempool.push(Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            100,
        ));
        blockchain.add_block();

        // Add more transactions to the mempool and add the second block
        blockchain.mempool.push(Transaction::new(
            "Bob".to_string(),
            "Charlie".to_string(),
            50,
        ));
        blockchain.mempool.push(Transaction::new(
            "Alice".to_string(),
            "Charlie".to_string(),
            50,
        ));
        blockchain.add_block();

        // Check the chain length
        assert_eq!(
            blockchain.chain.len(),
            3,
            "Blockchain should contain 3 blocks (genesis + 2 new blocks)"
        );

        // Validate the chain integrity
        assert!(
            blockchain.is_valid(),
            "Blockchain should be valid after adding multiple blocks"
        );

        // Check account balances
        assert_eq!(
            blockchain.accounts.get("Alice").unwrap(),
            &150,
            "Alice's balance should be updated correctly"
        );
        assert_eq!(
            blockchain.accounts.get("Bob").unwrap(),
            &50,
            "Bob's balance should be updated correctly"
        );
        assert_eq!(
            blockchain.accounts.get("Charlie").unwrap(),
            &100,
            "Charlie's balance should be updated correctly"
        );

        // Verify transactions in blocks
        let block_1 = &blockchain.chain[1];
        assert_eq!(
            block_1.transactions.len(),
            1,
            "Block 1 should contain 1 transaction"
        );
        assert_eq!(
            block_1.transactions[0],
            Transaction::new("Alice".to_string(), "Bob".to_string(), 100),
            "Block 1 transaction should match the expected transaction"
        );

        let block_2 = &blockchain.chain[2];
        assert_eq!(
            block_2.transactions.len(),
            2,
            "Block 2 should contain 2 transactions"
        );
        assert_eq!(
            block_2.transactions[0],
            Transaction::new("Bob".to_string(), "Charlie".to_string(), 50),
            "Block 2 first transaction should match the expected transaction"
        );
        assert_eq!(
            block_2.transactions[1],
            Transaction::new("Alice".to_string(), "Charlie".to_string(), 50),
            "Block 2 second transaction should match the expected transaction"
        );
    }
    #[test]
    fn high_volume_mempool_block_creation() {
        // Initialize the blockchain using the mock configuration
        let config = mock_config();
        let mut blockchain = Blockchain::new(config.clone()).unwrap();

        // Set initial balances
        blockchain.accounts.insert("Alice".to_string(), 1_000_000); // High balance to handle many transactions
        blockchain.accounts.insert("Bob".to_string(), 0);

        // Add a large number of transactions to the mempool
        let num_transactions = 10_000;
        for i in 0..num_transactions {
            blockchain.mempool.push(Transaction::new(
                "Alice".to_string(),
                format!("Bob_{}", i),
                1,
            ));
        }

        // Process the mempool and add a block
        let start_time = std::time::Instant::now();
        blockchain.add_block();
        let elapsed_time = start_time.elapsed();

        // Verify the block was added
        assert_eq!(
            blockchain.chain.len(),
            2,
            "Blockchain should contain 2 blocks (genesis + 1 new block)"
        );

        // Verify all transactions were added to the block
        let block = blockchain.chain.last().unwrap();
        assert_eq!(
            block.transactions.len(),
            num_transactions,
            "The new block should contain all transactions from the mempool"
        );

        // Verify remaining balance of Alice
        assert_eq!(
            blockchain.accounts.get("Alice").unwrap(),
            &(1_000_000 - num_transactions as u64),
            "Alice's balance should be reduced by the total amount transferred"
        );

        // Super arbitrary, refine when implementing network and max blocksize
        println!("Processed {} transactions in {:?}", num_transactions, elapsed_time);
        assert!(
            elapsed_time.as_secs() < 1,
            "Block creation should complete within 1 second"
        );
    }
    #[test]
    fn transaction_that_causes_balance_overflow_is_rejected() {
        // Initialize the blockchain using the mock configuration
        let config = mock_config();
        let mut blockchain = Blockchain::new(config.clone()).unwrap();

        // Set up an account with a balance near u64::MAX
        let near_max_balance = u64::MAX - 10;
        blockchain.accounts.insert("Alice".to_string(), near_max_balance);
        blockchain.accounts.insert("Bob".to_string(), 11);

        // Add a transaction that would cause Alice's balance to overflow
        blockchain.mempool.push(Transaction::new(
            "Bob".to_string(),
            "Alice".to_string(),
            11,
        ));

        // Attempt to add a block
        blockchain.add_block();

        // Ensure the blockchain length remains 1 (only the genesis block)
        assert_eq!(
            blockchain.chain.len(),
            1,
            "Blockchain should not add a block with an overflowing transaction"
        );

        // Ensure balances remain unchanged
        assert_eq!(
            blockchain.accounts.get("Alice").unwrap(),
            &near_max_balance,
            "Alice's balance should remain unchanged"
        );
        assert_eq!(
            blockchain.accounts.get("Bob").unwrap(),
            &11,
            "Bob's balance should remain unchanged"
        );

        // Ensure the mempool is cleared (invalid transactions should be removed)
        assert_eq!(
            blockchain.mempool.len(),
            0,
            "Mempool should be cleared after attempting to add a block"
        );

        // Ensure the blockchain is still valid
        assert!(
            blockchain.is_valid(),
            "Blockchain should still be valid"
        );
    }
}
