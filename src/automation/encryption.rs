use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct EncryptionService {
    encryption_key: Arc<RwLock<Option<[u8; 32]>>>,
}

impl EncryptionService {
    pub fn new() -> Self {
        Self {
            encryption_key: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn initialize(&self, key: [u8; 32]) {
        let mut enc_key = self.encryption_key.write().await;
        *enc_key = Some(key);
    }

    pub async fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut key);
        key
    }

    pub async fn encrypt_api_key(&self, api_key: &str) -> Result<(Vec<u8>, Vec<u8>), String> {
        let enc_key = self.encryption_key.read().await;
        let key = enc_key.ok_or("Encryption key not initialized")?;

        let cipher = Aes256Gcm::new(&Key::<Aes256Gcm>::from_slice(&key));
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, api_key.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;

        Ok((ciphertext, nonce.to_vec()))
    }

    pub async fn decrypt_api_key(
        &self,
        encrypted: &[u8],
        nonce: &[u8],
    ) -> Result<String, String> {
        let enc_key = self.encryption_key.read().await;
        let key = enc_key.ok_or("Encryption key not initialized")?;

        let cipher = Aes256Gcm::new(&Key::<Aes256Gcm>::from_slice(&key));
        let nonce = Nonce::from_slice(nonce);

        let plaintext = cipher
            .decrypt(nonce, encrypted)
            .map_err(|e| format!("Decryption failed: {}", e))?;

        String::from_utf8(plaintext).map_err(|e| format!("Invalid UTF-8: {}", e))
    }

    pub fn hash_api_key(api_key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl Default for EncryptionService {
    fn default() -> Self {
        Self::new()
    }
}
