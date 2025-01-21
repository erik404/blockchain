/// Represents a blockchain transaction.
#[derive(Debug, Clone)]
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
