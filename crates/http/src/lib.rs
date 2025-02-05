pub mod routes;
pub mod middleware;

use actix_web::{App, HttpServer};
use actix_web::web::Data;
use std::sync::Arc;
use tokio::sync::Mutex;
use locci_vault_core::vault::{Vault, VaultConfig};
use anyhow::Result;
use middleware::authentication::Authentication;

/// Starts the HTTP server with the given Vault instance and configuration.
pub async fn start_http_server(vault: Arc<Mutex<Vault>>, config: VaultConfig) -> Result<()> {
    HttpServer::new(move || {
        App::new()            
            // Use the custom authentication middleware.
            .wrap(Authentication)
            // Share the Vault instance with all request handlers.
            .app_data(Data::new(vault.clone()))
            // Configure routes (endpoints).
            .configure(routes::init_routes)
    })
    .bind(config.bind_address)?
    .run()
    .await?;
    Ok(())
}
