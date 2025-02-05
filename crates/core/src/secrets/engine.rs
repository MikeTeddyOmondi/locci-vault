use anyhow::Result;
use serde_json::Value;
use crate::vault::Secret;
use async_trait::async_trait;

/// A trait representing a secrets engine.
///
/// A secrets engine is responsible for dynamic secret generation and storage.
#[async_trait]
pub trait SecretsEngine: Send + Sync {
    /// Generate a secret based on the provided path and parameters.
    async fn generate(&self, path: &str, params: &Value) -> Result<Secret>;

    /// Store a secret at the given path.
    async fn store(&self, path: &str, secret: &Secret) -> Result<()>;
}

