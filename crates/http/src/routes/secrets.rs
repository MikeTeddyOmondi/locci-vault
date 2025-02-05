use actix_web::{post, get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use locci_vault_core::vault::Vault;
use std::sync::Arc;
use tokio::sync::Mutex;
use actix_web::web::{Data, Path, Json};

#[derive(Deserialize)]
pub struct SecretRequest {
    pub data: serde_json::Value,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub secret_id: Option<String>,
    pub error: Option<String>,
}

/// Endpoint to create a new secret.
/// The caller must provide an "Authorization" header with a valid token.
#[post("/secrets/{path}")]
pub async fn create_secret(
    path: Path<String>,
    req: Json<SecretRequest>,
    vault: Data<Arc<Mutex<Vault>>>,
    // Here we extract the full HTTP request to manually read the Authorization header.
    http_req: actix_web::HttpRequest,
) -> impl Responder {
    // Extract token from the "Authorization" header.
    let token = http_req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default()
        .to_string();

    let secret_data = match serde_json::to_vec(&req.data) {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                secret_id: None,
                error: Some(format!("Invalid secret data: {}", e)),
            });
        }
    };

    match vault.lock().await.create_secret(&path.into_inner(), &secret_data, &token).await {
        Ok(secret) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            secret_id: Some(secret.id.to_string()),
            error: None,
        }),
        Err(err) => HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            secret_id: None,
            error: Some(err.to_string()),
        }),
    }
}

/// Endpoint to read an existing secret.
/// The caller must provide an "Authorization" header with a valid token.
#[get("/secrets/{path}")]
pub async fn read_secret(
    path: Path<String>,
    vault: Data<Arc<Mutex<Vault>>>,
    http_req: actix_web::HttpRequest,
) -> impl Responder {
    let token = http_req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default()
        .to_string();

    match vault.lock().await.read_secret(&path.into_inner(), &token).await {
        Ok(secret) => HttpResponse::Ok().json(serde_json::json!({
            "path": secret.path,
            "id": secret.id.to_string(),
            "metadata": {
                "created_at": secret.metadata.created_at,
                "version": secret.metadata.version,
                "encrypted": secret.metadata.encrypted,
            }
        })),
        Err(err) => HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            secret_id: None,
            error: Some(err.to_string()),
        }),
    }
}
