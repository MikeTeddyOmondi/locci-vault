use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Simple CLI for interacting with the HTTP server
#[derive(Parser)]
#[command(name = "locci-vault-cli")]
#[command(about = "A CLI tool to interact with the Locci Vault HTTP server", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a request to the HTTP server
    SendRequest {
        /// API endpoint
        #[arg(short, long)]
        endpoint: String,
    },

    /// Check the health status of the server
    HealthCheck,
}

#[derive(Serialize, Deserialize)]
struct ResponseData {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let client = Client::new();
    let base_url = "http://127.0.0.1:8200/v1"; // Adjust based on your server configuration

    match cli.command {
        Commands::SendRequest { endpoint } => {
            let url = format!("{}/{}", base_url, endpoint);
            let res: ResponseData = client.get(&url).send().await?.json().await?;
            println!("Response: {}", res.message);
        }
        Commands::HealthCheck => {
            let url = format!("{}/health", base_url);
            let res: ResponseData = client.get(&url).send().await?.json().await?;
            println!("Server Health: {}", res.message);
        }
    }

    Ok(())
}
