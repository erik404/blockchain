use ripemd::Ripemd160;
use secp256k1::PublicKey;
use sha2::{Digest, Sha256};

/// Computes a blockchain address from a given public key.
pub fn compute_address_from_pub_key(public_key: &PublicKey) -> String {
    let public_key_bytes = public_key.serialize();
    let sha256_hash = Sha256::digest(public_key_bytes);
    // prevents public key recovery
    let ripemd160_hash = Ripemd160::digest(sha256_hash);
    // Convert to hex for simplicity
    hex::encode(ripemd160_hash)
}
