#[derive(Debug, Clone)]
pub(crate) struct Transaction {
    pub(crate) sender: String,
    pub(crate) receiver: String,
    pub(crate) amount: u64,
}

impl Transaction {
    pub(crate) fn new(sender: String, receiver: String, amount: u64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
        }
    }
}
