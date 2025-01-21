#[derive(Debug)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,        // Number of decimal places
    pub smallest_unit: u64, // Smallest unit of the token
    pub total_supply: u64,   // Total supply of the token
}

impl Token {
    pub fn new(name: String, symbol: String, decimals: u8, total_supply: u64) -> Self {
        let smallest_unit = 10u64.pow(decimals as u32);
        Token {
            name,
            symbol,
            decimals,
            smallest_unit,
            total_supply,
        }
    }

    pub fn format_amount(&self, amount: u64) -> String {
        let whole: u64 = amount / self.smallest_unit; // Get the whole number part
        let fractional: u64 = amount % self.smallest_unit; // Get the fractional part
        // Format fractional part with leading zeros
        format!(
            "{}.{:0width$}",
            whole,
            fractional,
            width = self.decimals as usize
        )
    }
}
