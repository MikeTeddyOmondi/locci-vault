pub mod secrets;
pub mod auth;

use actix_web::web;

/// Register all API endpoints.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(secrets::create_secret)
            .service(secrets::read_secret)
            .service(auth::login)
    );
}
