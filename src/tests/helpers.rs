use crate::config::{BlockchainConfig, Config, TokenConfig};

pub fn mock_config() -> Config {
    const TOKEN_NAME: &str = "test_name";
    const TOKEN_SYMBOL: &str = "test_symbol";
    const DECIMALS: u8 = 10;
    const TOTAL_SUPPLY: u64 = 21_000_000_000;
    const GENESIS_NAME: &str = "genesis_name";
    const DIFFICULTY: usize = 2;
    const GENESIS_PRE_MINED: u64 = 2_100_000;
    const GENESIS_MINER: &str = "Miner";
    
    Config {
        token: TokenConfig {
            name: TOKEN_NAME.to_string(),
            symbol: TOKEN_SYMBOL.to_string(),
            decimals: DECIMALS,
            total_supply: TOTAL_SUPPLY,
        },
        blockchain: BlockchainConfig {
            genesis_hash: GENESIS_NAME.to_string(),
            difficulty: DIFFICULTY,
            genesis_pre_mined: GENESIS_PRE_MINED,
            genesis_miner: GENESIS_MINER.to_string(),
        },
    }
}
