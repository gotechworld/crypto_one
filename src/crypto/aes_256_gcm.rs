use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};

use crate::crypto::Key;
use crate::error::CryptoError;
use crate::utils::NONCE_LEN;

pub fn encrypt(
    key: &Key,
    nonce: &[u8; NONCE_LEN],
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let cipher = Aes256Gcm::new_from_slice(key.as_bytes()).map_err(|_| CryptoError::InvalidKey)?;
    let nonce = Nonce::try_from(nonce.as_slice()).map_err(|_| CryptoError::Encryption)?;
    cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| CryptoError::Encryption)
}

#[cfg(test)]
pub fn decrypt(
    key: &Key,
    nonce: &[u8; NONCE_LEN],
    ciphertext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let cipher = Aes256Gcm::new_from_slice(key.as_bytes()).map_err(|_| CryptoError::InvalidKey)?;
    let nonce = Nonce::try_from(nonce.as_slice()).map_err(|_| CryptoError::Encryption)?;
    cipher
        .decrypt(&nonce, ciphertext)
        .map_err(|_| CryptoError::Encryption)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let key = Key::from_bytes([7u8; 32]);
        let nonce = [1u8; NONCE_LEN];
        let plaintext = b"Sensitive data payload.";

        let ciphertext = encrypt(&key, &nonce, plaintext).unwrap();
        assert_ne!(ciphertext.as_slice(), plaintext);

        let recovered = decrypt(&key, &nonce, &ciphertext).unwrap();
        assert_eq!(recovered, plaintext);
    }
}
