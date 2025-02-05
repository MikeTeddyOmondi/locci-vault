pub mod policy;
pub mod token;

use crate::vault::VaultConfig;
use anyhow::Result;

/// Authentication engine responsible for validating tokens.
#[derive(Clone)]
pub struct AuthEngine {
    pub config: VaultConfig,
    //  token_store: TokenStore,
    //  policy_engine: PolicyEngine,
    //  identity_store: IdentityStore,
}

impl AuthEngine {
    /// Creates a new authentication engine.
    pub async fn new(config: &VaultConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
        })
    }

    /// Validates the caller's token for a given secret path.
    /// (For demonstration, only tokens equal to "valid" are accepted.)
    pub async fn validate_token(&self, token: &str, _path: &str) -> Result<bool> {
        if token == "valid" {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
