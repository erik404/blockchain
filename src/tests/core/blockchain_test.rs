use super::*;
use crate::tests::helpers::mock_config;
use crate::Config;

#[test]
fn genesis_block_creation() {
    let config: Config = mock_config();
    let blockchain: Blockchain = Blockchain::new(config.clone()).unwrap();
    let block: &Block = blockchain.chain.last().unwrap();
    assert_eq!(
        block.index, 0,
        "Genesis index should be 0"
    );
    assert_eq!(
        block.previous_hash, config.blockchain.genesis_hash,
        "Genesis block hash should be equal to config.blockchain.genesis_hash"
    );
    assert_eq!(
        block.transactions, vec![],
        "Transactions should be empty"
    );
}
#[test]
fn initial_blockchain_values() {
    let config: Config = mock_config();
    let blockchain: Blockchain = Blockchain::new(config.clone()).unwrap();
    assert_eq!(
        blockchain.mempool, vec![],
        "Mempool should be empty"
    );
    assert_eq!(
        blockchain.difficulty, config.blockchain.difficulty,
        "Difficulty should be equal to config.blockchain.difficulty"
    );
    let mut accounts: HashMap<String, u64> = HashMap::new();
    accounts.insert(config.blockchain.genesis_miner, config.blockchain.genesis_pre_mined);
    assert_eq!(
        blockchain.accounts, accounts,
        "Accounts should hold genesis_miner and genesis_pre_mined"
    );
    let token: Token = Token::new(config.token.name, config.token.symbol, config.token.decimals, config.token.total_supply);
    assert_eq!(
        blockchain.token, token,
        "Token should be equal"
    );
    assert_eq!(
        blockchain.chain.len(), 1,
        "Blockchain should start with 1 block (genesis)"
    );
    assert!(
        blockchain.is_valid(),
        "Blockchain should be valid"
    );
}
#[test]
fn token_creation() {
    let config: Config = mock_config();
    let blockchain: Blockchain = Blockchain::new(config.clone()).unwrap();
    assert_eq!(
        blockchain.token.name, config.token.name,
        "Token name does not match the configuration"
    );
    assert_eq!(
        blockchain.token.total_supply, config.token.total_supply,
        "Token total_supply does not match the configuration"
    );
    assert_eq!(
        blockchain.token.symbol, config.token.symbol,
        "Token symbol does not match the configuration"
    );
    assert_eq!(
        blockchain.token.decimals, config.token.decimals,
        "Token decimals does not match the configuration"
    );
    assert_eq!(
        blockchain.token.smallest_unit, 10u64.pow(blockchain.token.decimals as u32),
        "Token decimals does not match the configuration"
    );
}
#[test]
fn genesis_pre_mined_can_not_overflow_total_supply() {
    let mut config: Config = mock_config();
    config.blockchain.genesis_pre_mined = config.token.total_supply +1;
    let blockchain_result: Result<Blockchain, String> = Blockchain::new(config.clone());
    assert_eq!(
        blockchain_result.unwrap_err(),
        "ERR_TOTAL_SUPPLY_LESS_THAN_PRE_MINED"
    );
}