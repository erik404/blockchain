use serde::Deserialize;
use std::path::PathBuf;
use std::{env, fs};

/// Configuration for the token.
#[derive(Debug, Deserialize, Clone)]
pub struct TokenConfig {
    pub name: String,      // Token name (e.g., "TestCoin")
    pub symbol: String,    // Token symbol (e.g., "TST")
    pub decimals: u8,      // Number of decimal places (e.g., 8 for Bitcoin)
    pub total_supply: u64, // Total supply of the token
}

/// Configuration for the blockchain.
#[derive(Debug, Deserialize, Clone)]
pub struct BlockchainConfig {
    pub genesis_hash: String,   // Name of the genesis block
    pub difficulty: usize,      // Mining difficulty level
    pub genesis_pre_mined: u64, // The token amount that is mined at Genesis
    pub genesis_miner: String,  // The name or account that mined Genesis
}

/// Main configuration struct combining token and blockchain settings.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub token: TokenConfig,           // Token configuration
    pub blockchain: BlockchainConfig, // Blockchain configuration
}

impl Config {
    /// Loads configuration from a `config.yml` file in the current directory.
    pub fn load() -> Self {
        // Construct the path to the config file
        let mut config_path: PathBuf = match env::current_dir() {
            Ok(p) => p,
            Err(why) => {
                eprintln!("Error: Unable to read current directory: {}", why);
                std::process::exit(1); // Exit on failure to determine the directory
            }
        };
        config_path.push("config.yml");

        // Read the config file's content
        let contents: String = match fs::read_to_string(&config_path) {
            Ok(contents) => contents,
            Err(why) => {
                eprintln!(
                    "Error: Could not read the config file at {:?}. Ensure the file exists and is readable: {}",
                    config_path, why
                );
                std::process::exit(1); // Exit on failure to read the file
            }
        };

        // Deserialize the YAML content into the Config struct
        let config: Self = match serde_yaml::from_str(&contents) {
            Ok(config) => config,
            Err(why) => {
                eprintln!(
                    "Error: Could not parse the config file at {:?}. Ensure the file is valid YAML: {}",
                    config_path, why
                );
                std::process::exit(1); // Exit on failure to parse the YAML
            }
        };

        config
    }
}
