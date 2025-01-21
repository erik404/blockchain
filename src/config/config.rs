use serde::Deserialize;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Deserialize)]
pub struct TokenConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
}

#[derive(Debug, Deserialize)]
pub struct BlockchainConfig {
    pub genesis_name: String,
    pub difficulty: usize,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: TokenConfig,
    pub blockchain: BlockchainConfig,
}

impl Config {
    pub fn load() -> Self {
        // Construct the config path dynamically
        let mut config_path: PathBuf = match env::current_dir() {
            Ok(p) => p,
            Err(why) => panic!("Error: Unable to read current directory: {}", why),
        };
        config_path.push("config.yml");
        // Read the file content
        let contents: String = match fs::read_to_string(&config_path) {
            Ok(contents) => contents,
            Err(why) => {
                panic!("Error: Could not read the config file at {:?}. Ensure the file exists and is readable: {}", config_path, why);
            }
        };
        // Deserialize the YAML content
        let config: Self = match serde_yaml::from_str(&contents) {
            Ok(config) => config,
            Err(why) => {
                panic!("Error: Could not parse the config file at {:?}. Ensure the file is valid YAML: {}", config_path, why);
            }
        };
        config
    }
}
