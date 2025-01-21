#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
        }
    }
    pub fn stringify(&self) -> String {
        format!("{}{}{}", self.sender, self.receiver, self.amount)
    }
}
