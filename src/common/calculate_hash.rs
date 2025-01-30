use crate::core::transaction::Transaction;
use sha2::{Digest, Sha256};

/// Calculates the hash of the block based on its properties.
pub fn calculate_block_hash(
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
        transactions_string(transactions),
        previous_hash,
        nonce
    );

    let mut hasher = Sha256::new();
    hasher.update(input);
    hex::encode(hasher.finalize())
}

/// Converts the list of transactions into a string for hashing.
fn transactions_string(transactions: &Vec<Transaction>) -> String {
    let mut transactions_string: String = String::new();
    for transaction in transactions {
        transactions_string.push_str(transaction.stringify().as_str());
    }
    transactions_string
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::transaction::Transaction;
    #[test]
    fn deterministic_and_unique_block_hash() {
        let index = 1;
        let timestamp = "2025-01-01T00:00:00Z";
        let transactions = vec![
            Transaction::new("Alice".to_string(), "Bob".to_string(), 100),
            Transaction::new("Charlie".to_string(), "Dave".to_string(), 200),
        ];
        let previous_hash = "0000000000000000000000000000000000000000000000000000000000000000";
        let nonce = 12345;

        let block_hash =
            calculate_block_hash(index, timestamp, &transactions, previous_hash, nonce);

        // Check that the hash has the expected length (SHA256 is 64 hex characters)
        assert_eq!(
            block_hash.len(),
            64,
            "Block hash should be 64 characters long (SHA256)"
        );

        // Hash is deterministic (same input should produce the same hash)
        let block_hash_recalculated =
            calculate_block_hash(index, timestamp, &transactions, previous_hash, nonce);
        assert_eq!(
            block_hash, block_hash_recalculated,
            "Hash calculation should be deterministic for the same input"
        );

        //  A small change in the input results in a different hash
        let different_hash =
            calculate_block_hash(index + 1, timestamp, &transactions, previous_hash, nonce);
        assert_ne!(
            block_hash, different_hash,
            "Hashes should be different for different block indices"
        );
    }
    #[test]
    fn transactions_string_concatenation() {
        // Arrange: Create a list of transactions
        let transactions = vec![
            Transaction::new("Alice".to_string(), "Bob".to_string(), 100),
            Transaction::new("Charlie".to_string(), "Dave".to_string(), 200),
        ];

        // Act: Convert transactions to a string
        let transactions_str = transactions_string(&transactions);

        // Assert: Check the expected concatenated string
        let expected_str = format!(
            "{}{}",
            transactions[0].stringify(),
            transactions[1].stringify()
        );
        assert_eq!(
            transactions_str, expected_str,
            "Transaction string should concatenate all transaction string representations"
        );
    }
}
