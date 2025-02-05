use ring::rand::{SystemRandom, SecureRandom};
use locci_vault_http::start_http_server;
use locci_vault_core::VaultConfig;
use tokio::sync::Mutex;
use std::io::Write;
use std::sync::Arc;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing/logging
    tracing_subscriber::fmt::init();

    // Configuration
    let config = VaultConfig {
        bind_address: "127.0.0.1:8200".to_string(),
        storage_path: "/var/lib/locci-vault".to_string(),
        encryption_key: generate_encryption_key(),
    };

    // Create Vault instance
    let vault = Arc::new(Mutex::new(
        locci_vault_core::Vault::new(config.clone()).await?
    ));

    // Start HTTP server
    start_http_server(vault, config).await?;

    Ok(())
}

fn generate_encryption_key() -> Vec<u8> {
    // TODO: [prod] use a secure key management system    
    let rng = SystemRandom::new();
    let mut key = [0u8; 32]; // Define a mutable buffer
    rng.fill(&mut key).expect("Failed to generate random bytes");
    // ring::rand::generate::<[u8; 32]>(SecureRandom::fill(&self, &mut key)).unwrap().to_vec()
    println!("Generated Key: {:?}", base64::encode(&key));
    save_encryption_key_to_file(&Vec::from(&key), "encryption_key.bin").expect("Failed to save key");
    Vec::from(key)
}


fn save_encryption_key_to_file(key: &Vec<u8>, file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(&key)?;
    Ok(())
}