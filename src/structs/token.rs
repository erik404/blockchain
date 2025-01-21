/// Represents a cryptocurrency token in the blockchain.
#[derive(Debug)]
pub struct Token {
    pub name: String,       // Name of the token (e.g., "TestCoin")
    pub symbol: String,     // Symbol of the token (e.g., "TST")
    pub decimals: u8,       // Number of decimal places (e.g., 8 for Bitcoin)
    pub smallest_unit: u64, // Smallest unit of the token (e.g., 1 satoshi for Bitcoin)
    pub total_supply: u64,  // Total supply of the token in smallest units
}

impl Token {
    /// Creates a new Token instance.
    /// - Calculates the smallest unit based on the number of decimals.
    pub fn new(name: String, symbol: String, decimals: u8, total_supply: u64) -> Self {
        let smallest_unit = 10u64.pow(decimals as u32); // Calculate smallest unit (10^decimals)
        Token {
            name,
            symbol,
            decimals,
            smallest_unit,
            total_supply,
        }
    }

    /// Formats a token amount (in smallest units) as a human-readable string.
    /// - Converts the smallest unit to a whole number and fractional part.
    pub fn format_amount(&self, amount: u64) -> String {
        let whole: u64 = amount / self.smallest_unit; // Whole number part
        let fractional: u64 = amount % self.smallest_unit; // Fractional part
        format!(
            "{}.{:0width$}", // Format with leading zeros for fractional part
            whole,
            fractional,
            width = self.decimals as usize
        )
    }
}
