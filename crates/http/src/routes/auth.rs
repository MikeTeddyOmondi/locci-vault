use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use locci_vault_core::auth::token::generate_token;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

/// A simple login endpoint that returns a token.
/// In a production system, you would validate the credentials properly.
#[post("/login")]
pub async fn login(info: web::Json<LoginRequest>) -> impl Responder {
    // For demonstration, any credentials will yield the same token.
    let token = generate_token();
    HttpResponse::Ok().json(LoginResponse { token })
}
