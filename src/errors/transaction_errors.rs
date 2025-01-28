#[derive(Debug, PartialEq)]
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
    BalanceOverflow,
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
            TransactionError::BalanceOverflow => {
                write!(f, "Transaction rejected: Balance overflow.")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transaction_error_message_formatting() {
        let error = TransactionError::AddressCannotBeEmpty;
        assert_eq!(
            format!("{}", error),
            "Transaction address cannot be empty.",
            "Display output for AddressCannotBeEmpty is incorrect"
        );

        let error = TransactionError::BalanceOverflow;
        assert_eq!(
            format!("{}", error),
            "Transaction rejected: Balance overflow.",
            "Display output for BalanceOverflow is incorrect"
        );

        let error = TransactionError::SenderAndReceiverCannotBeTheSame;
        assert_eq!(
            format!("{}", error),
            "Sender and receiver cannot be the same.",
            "Display output for SenderAndReceiverCannotBeTheSame is incorrect"
        );

        let error = TransactionError::AmountMustBeGreaterThanZero;
        assert_eq!(
            format!("{}", error),
            "Transaction must be greater than zero.",
            "Display output for AmountMustBeGreaterThanZero is incorrect"
        );

        let error = TransactionError::InsufficientBalance {
            sender: "Alice".to_string(),
            requested: 100,
            available: 50,
        };
        assert_eq!(
            format!("{}", error),
            "Transaction rejected: Alice has insufficient balance (100 requested, 50 available).",
            "Display output for InsufficientBalance is incorrect"
        );

        let error = TransactionError::SenderDoesNotExist {
            sender: "Alice".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Transaction rejected: Sender Alice does not exist.",
            "Display output for SenderDoesNotExist is incorrect"
        );
    }
}
