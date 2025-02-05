use crate::vault::Secret;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A simple in-memory storage backend for secrets.
/// TODO: Replace this with a persistent backend for production.
#[derive(Clone)]
pub struct StorageBackend {
    pub secrets: Arc<RwLock<HashMap<String, Secret>>>,
    // physical: Box<dyn PhysicalStorage>,
    // logical: LogicalStorage,
}

// Implementation for StorageBackend
impl StorageBackend {
    /// Creates a new storage backend.
    pub async fn new(_config: &crate::vault::VaultConfig) -> Result<Self> {
        Ok(Self {
            secrets: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Stores a secret.
    pub async fn store_secret(&mut self, secret: &Secret) -> Result<()> {
        let mut store = self.secrets.write().await;
        store.insert(secret.path.clone(), secret.clone());
        Ok(())
    }

    /// Reads a secret from storage.
    pub async fn read_secret(&self, path: &str) -> Result<Secret> {
        let store = self.secrets.read().await;
        if let Some(secret) = store.get(path) {
            Ok(secret.clone())
        } else {
            Err(anyhow!("Secret not found"))
        }
    }
}

