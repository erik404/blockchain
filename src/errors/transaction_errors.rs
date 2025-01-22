#[derive(Debug)]
pub enum TransactionError {
    AddressCannotBeEmpty,
    SenderAndReceiverCannotBeTheSame,
    AmountMustBeGreaterThanZero,
    InsufficientBalance {
        sender: String,
        requested: u64,
        available: u64,
    },
    SenderDoesNotExist {
        sender: String,
    },
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::AddressCannotBeEmpty => {
                write!(f, "Transaction address cannot be empty.")
            }
            TransactionError::SenderAndReceiverCannotBeTheSame => {
                write!(f, "Sender and receiver cannot be the same.")
            }
            TransactionError::AmountMustBeGreaterThanZero => {
                write!(f, "Transaction must be greater than zero.")
            }
            TransactionError::InsufficientBalance {
                sender,
                requested,
                available,
            } => {
                write!(
                    f,
                    "Transaction rejected: {} has insufficient balance ({} requested, {} available).",
                    sender, requested, available
                )
            }
            TransactionError::SenderDoesNotExist { sender } => {
                write!(f, "Transaction rejected: Sender {} does not exist.", sender)
            }
        }
    }
}
