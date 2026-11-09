use std::fmt;

#[derive(Debug)]
pub enum CryptoError {
    InvalidPassphrase,
    InvalidKey,
    Encryption,
    Mac,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPassphrase => write!(f, "passphrase must not be empty"),
            Self::InvalidKey => write!(f, "encryption key has an invalid length"),
            Self::Encryption => write!(f, "AES-256-GCM encryption failed"),
            Self::Mac => write!(f, "HMAC-SHA256 computation failed"),
        }
    }
}

impl std::error::Error for CryptoError {}
