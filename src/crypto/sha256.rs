use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

use crate::crypto::Key;
use crate::error::CryptoError;

type HmacSha256 = Hmac<Sha256>;

/// HMAC-SHA256 over `data`, keyed by the derived encryption key.
pub fn compute(key: &Key, data: &[u8]) -> Result<[u8; 32], CryptoError> {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).map_err(|_| CryptoError::Mac)?;
    mac.update(data);
    let tag = mac.finalize().into_bytes();
    let mut out = [0u8; 32];
    out.copy_from_slice(tag.as_slice());
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mac_changes_when_data_changes() {
        let key = Key::from_bytes([9u8; 32]);
        let first = compute(&key, b"a").unwrap();
        let second = compute(&key, b"b").unwrap();
        assert_ne!(first, second);
    }
}
