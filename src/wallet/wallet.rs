use crate::common::compute_address_from_pub_key::compute_address_from_pub_key;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (private_key, public_key) = secp.generate_keypair(&mut secp256k1::rand::thread_rng());
        Wallet {
            private_key,
            public_key,
        }
    }
    /// Uses the standalone function to get the wallet address.
    pub fn get_address(&self) -> String {
        compute_address_from_pub_key(&self.public_key)
    }

    /// Signs transaction data using the private key.
    pub fn sign_transaction(&self, data: &str) -> String {
        let secp = Secp256k1::new();
        let message_hash = Sha256::digest(data.as_bytes());
        let message = secp256k1::Message::from_digest(message_hash.0);
        secp.sign_ecdsa(&message, &self.private_key).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_wallet_has_address() {
        let wallet = Wallet::new();
        assert!(!wallet.get_address().is_empty());
    }

    #[test]
    fn new_wallet_addresses_are_unique() {
        let wallet1 = Wallet::new();
        let wallet2 = Wallet::new();
        assert_ne!(wallet1.get_address(), wallet2.get_address());
    }
}
