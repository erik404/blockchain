#[derive(Debug, Clone)]
pub(crate) struct Transaction {
    pub(crate) value: String,
}

impl Transaction {
    pub(crate) fn new(value: String) -> Self {
        Transaction { value }
    }
}
