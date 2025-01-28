use serde::Deserialize;
use std::{env, fs, path::Path};

/// Configuration for the token.
#[derive(Debug, Deserialize, Clone)]
pub struct TokenConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
}

/// Configuration for the blockchain.
#[derive(Debug, Deserialize, Clone)]
pub struct BlockchainConfig {
    pub genesis_hash: String,
    pub difficulty: usize,
    pub genesis_pre_mined: u64,
    pub genesis_miner: String,
}

/// Main configuration struct combining token and blockchain settings.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub token: TokenConfig,
    pub blockchain: BlockchainConfig,
}

impl Config {
    /// Loads configuration from a specified path.
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let contents = fs::read_to_string(&path).map_err(|e| {
            format!(
                "Could not read the config file at {:?}: {}",
                path.as_ref(),
                e
            )
        })?;

        serde_yaml::from_str(&contents).map_err(|e| {
            format!(
                "Could not parse the config file at {:?}: {}",
                path.as_ref(),
                e
            )
        })
    }

    /// Loads configuration from a `config.yml` file in the current directory.
    pub fn load() -> Result<Self, String> {
        let mut config_path =
            env::current_dir().map_err(|e| format!("Unable to read current directory: {}", e))?;
        config_path.push("config.yml");
        Self::load_from_path(config_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;
    #[test]
    fn valid_config_loads_successfully() {
        let config_content = r#"
token:
  name: "TestToken"
  symbol: "TT"
  decimals: 8
  total_supply: 1000000000
blockchain:
  genesis_hash: "00000000000000000000000000000000"
  difficulty: 2
  genesis_pre_mined: 500000000
  genesis_miner: "Miner1"
"#;
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        fs::write(temp_file.path(), config_content).expect("Failed to write to temp file");

        let config = Config::load_from_path(temp_file.path());

        assert!(config.is_ok(), "Valid config should load successfully");
        let config = config.unwrap();
        assert_eq!(config.token.name, "TestToken");
        assert_eq!(config.token.symbol, "TT");
        assert_eq!(config.token.decimals, 8);
        assert_eq!(config.token.total_supply, 1_000_000_000);
        assert_eq!(
            config.blockchain.genesis_hash,
            "00000000000000000000000000000000"
        );
        assert_eq!(config.blockchain.difficulty, 2);
        assert_eq!(config.blockchain.genesis_pre_mined, 500_000_000);
        assert_eq!(config.blockchain.genesis_miner, "Miner1");
    }
    #[test]
    fn missing_yaml_config_file_returns_error() {
        let result = Config::load_from_path("nonexistent_config.yml");

        assert!(
            result.is_err(),
            "Loading a non-existent config file should return an error"
        );
        let error_message = result.unwrap_err();
        assert!(
            error_message.contains("Could not read the config file"),
            "Error message should indicate that the config file is missing"
        );
    }
    #[test]
    fn invalid_yaml_config_file_returns_error() {
        let config_content = r#"
token:
  name: "TestToken"
  symbol: "TT"
  decimals: "invalid" # Invalid type
  total_supply: 1000000000
blockchain:
  genesis_hash: "00000000000000000000000000000000"
  difficulty: 2
  genesis_pre_mined: 500000000
  genesis_miner: "Miner1"
"#;
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        fs::write(temp_file.path(), config_content).expect("Failed to write to temp file");

        let result = Config::load_from_path(temp_file.path());

        assert!(
            result.is_err(),
            "Loading an invalid YAML config file should return an error"
        );
        let error_message = result.unwrap_err();
        assert!(
            error_message.contains("Could not parse the config file"),
            "Error message should indicate invalid YAML parsing"
        );
    }
    #[test]
    fn missing_key_in_config_file_returns_error() {
        let config_content = r#"
token:
  # name: "TestToken" # Missing key
  symbol: "TT"
  decimals: 8
  total_supply: 1000000000
blockchain:
  genesis_hash: "00000000000000000000000000000000"
  difficulty: 2
  genesis_pre_mined: 500000000
  genesis_miner: "Miner1"
"#;
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        fs::write(temp_file.path(), config_content).expect("Failed to write to temp file");

        let result = Config::load_from_path(temp_file.path());

        assert!(
            result.is_err(),
            "Loading a config file with a missing key should return an error"
        );
        let error_message = result.unwrap_err();
        assert!(
            error_message.contains("Could not parse the config file"),
            "Error message should indicate a missing key during YAML parsing"
        );
    }
}
