use anyhow::{anyhow, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::auth::AuthEngine;
use crate::storage::StorageBackend;
use crate::crypto::encryption::{encrypt_secret, decrypt_secret};

/// Vault configuration parameters.
#[derive(Clone, Debug)]
pub struct VaultConfig {
    pub bind_address: String,
    pub storage_path: String,
    pub encryption_key: Vec<u8>,
}

/// Structure representing a stored secret.
#[derive(Clone, Debug)]
pub struct Secret {
    pub id: Uuid,
    pub path: String,
    pub data: Vec<u8>,
    pub metadata: SecretMetadata,
}

/// Metadata associated with a secret.
#[derive(Clone, Debug)]
pub struct SecretMetadata {
    pub created_at: DateTime<Utc>,
    pub version: u64,
    pub encrypted: bool,
}

/// Main Vault structure integrating auth, storage, and crypto.
#[derive(Clone)]
pub struct Vault {
    pub config: VaultConfig,
    pub auth: Arc<Mutex<AuthEngine>>,
    pub storage: Arc<Mutex<StorageBackend>>,
}

impl Vault {
    /// Creates a new Vault instance.
    pub async fn new(config: VaultConfig) -> Result<Self> {
        let auth = Arc::new(Mutex::new(AuthEngine::new(&config).await?));
        let storage = Arc::new(Mutex::new(StorageBackend::new(&config).await?));
        Ok(Self { config, auth, storage })
    }

    /// Creates a new secret after verifying authorization.
    pub async fn create_secret(
        &self,
        path: &str,
        data: &[u8],
        caller_token: &str,
    ) -> Result<Secret> {
        // Authorization check.
        let authorized = self.auth.lock().await.validate_token(caller_token, path).await?;
        if !authorized {
            return Err(anyhow!("Unauthorized access"));
        }

        // Encrypt secret data.
        let encrypted_data = encrypt_secret(data, &self.config.encryption_key)?;

        // Create the secret with metadata.
        let secret = Secret {
            id: Uuid::new_v4(),
            path: path.to_string(),
            data: encrypted_data,
            metadata: SecretMetadata {
                created_at: Utc::now(),
                version: 1,
                encrypted: true,
            },
        };

        // Store the secret using the storage backend.
        self.storage.lock().await.store_secret(&secret).await?;
        Ok(secret)
    }

    /// Reads a secret (after authorization) and decrypts it.
    pub async fn read_secret(&self, path: &str, caller_token: &str) -> Result<Secret> {
        // Authorization check.
        let authorized = self.auth.lock().await.validate_token(caller_token, path).await?;
        if !authorized {
            return Err(anyhow!("Unauthorized access"));
        }

        // Retrieve the secret from the storage backend.
        let mut secret = self.storage.lock().await.read_secret(path).await?;
        // Decrypt the secret data.
        let decrypted_data = decrypt_secret(&secret.data, &self.config.encryption_key)?;
        secret.data = decrypted_data;
        Ok(secret)
    }
}
