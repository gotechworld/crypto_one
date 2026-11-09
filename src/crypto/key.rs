use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

use crate::error::CryptoError;

const PBKDF2_ROUNDS: u32 = 600_000;
const SALT_LEN: usize = 16;

/// 256-bit key derived with PBKDF2-HMAC-SHA256.
pub struct Key {
    bytes: [u8; 32],
}

impl Key {
    pub fn derive_from_passphrase(passphrase: &str) -> Result<Self, CryptoError> {
        if passphrase.is_empty() {
            return Err(CryptoError::InvalidPassphrase);
        }

        let mut salt = [0u8; SALT_LEN];
        rand::fill(&mut salt);

        let mut bytes = [0u8; 32];
        pbkdf2_hmac::<Sha256>(passphrase.as_bytes(), &salt, PBKDF2_ROUNDS, &mut bytes);
        Ok(Self { bytes })
    }

    #[cfg(test)]
    pub(crate) fn from_bytes(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }

    pub(crate) fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Key([redacted])")
    }
}
