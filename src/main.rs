mod crypto;
mod error;
mod utils;

use crate::crypto::{Key, aes_256_gcm, sha256};
use crate::error::CryptoError;
use crate::utils::generate_nonce;

fn main() -> Result<(), CryptoError> {
    // 1. Generate an encryption key from a passphrase
    let key = Key::derive_from_passphrase("super_secure_passphrase")?;

    // 2. Perform AES-256-GCM encryption
    let plaintext = b"Sensitive data payload.";
    let nonce = generate_nonce()?;
    let ciphertext = aes_256_gcm::encrypt(&key, &nonce, plaintext)?;

    // 3. MAC the ciphertext (borrowed, so it can still be printed)
    let mac = sha256::compute(&key, &ciphertext)?;

    println!("Ciphertext: {}", hex::encode(&ciphertext));
    println!("MAC: {}", hex::encode(mac));
    Ok(())
}
