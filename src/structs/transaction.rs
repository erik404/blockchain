/// Represents a blockchain transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub sender: String,   // Address of the sender
    pub receiver: String, // Address of the receiver
    pub amount: u64,      // Amount to be transferred (in smallest units)
}

impl Transaction {
    /// Creates a new transaction with the specified sender, receiver, and amount.
    pub fn new(sender: String, receiver: String, amount: u64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
        }
    }

    /// Converts the transaction into a string format for hashing or serialization.
    pub fn stringify(&self) -> String {
        format!("{}{}{}", self.sender, self.receiver, self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_new_transaction() {
        // Arrange: Set up sender, receiver, and amount
        let sender = "Alice".to_string();
        let receiver = "Bob".to_string();
        let amount = 100;

        let transaction = Transaction::new(sender.clone(), receiver.clone(), amount);

        assert_eq!(
            transaction.sender, sender,
            "Sender should match the provided value"
        );
        assert_eq!(
            transaction.receiver, receiver,
            "Receiver should match the provided value"
        );
        assert_eq!(
            transaction.amount, amount,
            "Amount should match the provided value"
        );
    }
    #[test]
    fn transaction_stringify() {
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 100);
        let transaction_string = transaction.stringify();

        assert_eq!(
            transaction_string, "AliceBob100",
            "Stringified transaction should concatenate sender, receiver, and amount"
        );
    }
}
