use crate::error::CryptoError;

pub const NONCE_LEN: usize = 12;

/// 96-bit nonce for AES-GCM. A fresh value is required for every encryption under the same key.
pub fn generate_nonce() -> Result<[u8; NONCE_LEN], CryptoError> {
    let mut nonce = [0u8; NONCE_LEN];
    rand::fill(&mut nonce);
    Ok(nonce)
}
