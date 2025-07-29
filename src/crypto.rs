use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use anyhow::{Context, Result};
use argon2::{self, Argon2};
use base64::{engine::general_purpose::STANDARD, Engine as _};

const SALT_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;

pub struct Crypto;

impl Crypto {
    pub fn encrypt(plaintext: &str, password: &str) -> Result<EncryptedData> {
        let mut rng = OsRng;
        let key = Aes256Gcm::generate_key(&mut rng);
        let nonce = Aes256Gcm::generate_nonce(&mut rng);

        let salt = key[..SALT_LENGTH].to_vec();
        let nonce_bytes = nonce.as_slice().to_vec();

        let password_key_bytes = Self::derive_key_from_password(password, &salt);
        let password_key = Key::<Aes256Gcm>::from_slice(&password_key_bytes);

        let cipher = Aes256Gcm::new(password_key);

        let ciphertext = cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        Ok(EncryptedData::new(salt, nonce_bytes, ciphertext))
    }

    pub fn decrypt(encrypted_data: &EncryptedData, password: &str) -> Result<String> {
        let key_bytes = Self::derive_key_from_password(password, encrypted_data.salt());
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(encrypted_data.nonce());

        let plaintext = cipher
            .decrypt(nonce, encrypted_data.ciphertext())
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        String::from_utf8(plaintext).context("Invalid UTF-8 in decrypted data")
    }

    fn derive_key_from_password(password: &str, salt: &[u8]) -> [u8; 32] {
        let mut key = [0u8; 32];
        let argon2 = Argon2::default();
        argon2
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .expect("Key derivation failed");
        key
    }
}

#[derive(Debug)]
pub struct EncryptedData {
    salt: Vec<u8>,
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
}

impl EncryptedData {
    pub fn new(salt: Vec<u8>, nonce: Vec<u8>, ciphertext: Vec<u8>) -> Self {
        Self {
            salt,
            nonce,
            ciphertext,
        }
    }

    pub fn salt(&self) -> &[u8] {
        &self.salt
    }

    pub fn nonce(&self) -> &[u8] {
        &self.nonce
    }

    pub fn ciphertext(&self) -> &[u8] {
        &self.ciphertext
    }

    pub fn to_base64(&self) -> String {
        let mut combined = Vec::new();
        combined.extend_from_slice(&self.salt);
        combined.extend_from_slice(&self.nonce);
        combined.extend_from_slice(&self.ciphertext);
        STANDARD.encode(combined)
    }

    pub fn from_base64(encoded: &str) -> Result<Self> {
        let decoded = STANDARD
            .decode(encoded)
            .context("Invalid base64 encoding")?;

        if decoded.len() < SALT_LENGTH + NONCE_LENGTH {
            return Err(anyhow::anyhow!("Encrypted data too short"));
        }

        let salt = decoded[0..SALT_LENGTH].to_vec();
        let nonce = decoded[SALT_LENGTH..SALT_LENGTH + NONCE_LENGTH].to_vec();
        let ciphertext = decoded[SALT_LENGTH + NONCE_LENGTH..].to_vec();

        Ok(Self::new(salt, nonce, ciphertext))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt() {
        let message = "Secret Message!";
        let password = "password";

        let encrypted = Crypto::encrypt(message, password).unwrap();

        let encoded = encrypted.to_base64();
        let decoded = EncryptedData::from_base64(&encoded).unwrap();

        let decrypted = Crypto::decrypt(&decoded, password).unwrap();

        assert_eq!(message, decrypted);
    }

    #[test]
    fn wrong_password() {
        let message = "Secret Message!";
        let password = "correct_password";
        let wrong_password = "wrong_password";

        let encrypted = Crypto::encrypt(message, password).unwrap();
        let result = Crypto::decrypt(&encrypted, wrong_password);

        assert!(result.is_err());
    }

    #[test]
    fn struct_accessors() {
        let message = "Secret Message!";
        let password = "password";

        let encrypted = Crypto::encrypt(message, password).unwrap();

        assert_eq!(encrypted.salt().len(), SALT_LENGTH);
        assert_eq!(encrypted.nonce().len(), NONCE_LENGTH);
        assert!(!encrypted.ciphertext().is_empty());
    }

    #[test]
    fn base64_encoding() {
        let message = "Secret Message!";
        let password = "password";

        let encrypted = Crypto::encrypt(message, password).unwrap();
        let base64_str = encrypted.to_base64();

        assert!(!base64_str.is_empty());

        let decoded = EncryptedData::from_base64(&base64_str).unwrap();
        let decrypted = Crypto::decrypt(&decoded, password).unwrap();

        assert_eq!(message, decrypted);
    }
}
