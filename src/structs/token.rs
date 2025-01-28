/// Represents a cryptocurrency token in the blockchain.
#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub smallest_unit: u64,
    pub total_supply: u64,
}

impl Token {
    /// Creates a new Token instance.
    /// - Calculates the smallest unit based on the number of decimals.
    pub fn new(name: String, symbol: String, decimals: u8, total_supply: u64) -> Self {
        let smallest_unit: u64 = 10u64.pow(decimals as u32); // Calculate smallest unit (10^decimals)
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
        let whole: u64 = amount / self.smallest_unit;
        let fractional: u64 = amount % self.smallest_unit;
        format!(
            "{}.{:0width$}",
            whole,
            fractional,
            width = self.decimals as usize
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::mock_config;
    #[test]
    fn format_token_amount() {
        let config = mock_config();
        let token = Token::new(
            config.token.name,
            config.token.symbol,
            8,
            config.token.total_supply,
        );

        // Amount with no fractional part
        assert_eq!(
            token.format_amount(10 * token.smallest_unit),
            "10.00000000",
            "Formatting whole number amount should match"
        );

        // Amount with fractional part
        assert_eq!(
            token.format_amount(12345678901),
            "123.45678901",
            "Formatting amount with fractional part should match"
        );

        // Amount less than one whole unit
        assert_eq!(
            token.format_amount(987),
            "0.00000987",
            "Formatting amount smaller than one whole unit should match"
        );

        // Amount of zero
        assert_eq!(
            token.format_amount(0),
            "0.00000000",
            "Formatting zero amount should return correctly formatted string"
        );

        // Large amount
        assert_eq!(
            token.format_amount(123456789012345678),
            "1234567890.12345678",
            "Formatting large amount should match"
        );
    }
}
