use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::task::{Context, Poll};
use futures::FutureExt; 
// use std::pin::Pin;
use std::rc::Rc;

/// A middleware for checking the presence of an "Authorization" header.
pub struct Authentication;

impl<S> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<actix_web::body::BoxBody>, Error = actix_web::Error>
        + 'static,
{
    type Transform = AuthenticationMiddleware<S>;
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let path = req.path(); // Get request path

        // Allow login requests to pass without authentication
        if path == "/api/v1/login" || path == "/v1/login" {
            return Box::pin(svc.call(req).map(|res| res.map(|r| r.map_into_boxed_body())));
        }
        
        Box::pin(async move {
            // Check authentication (example check, replace with actual logic)
            if !req.headers().contains_key("Authorization") {
                let res = HttpResponse::Unauthorized().finish();
                let service_response = req.into_response(res.map_into_boxed_body());
                return Ok(service_response);
            }

            // Call next service in middleware chain, ensuring correct type
            svc.call(req).await.map(|res| res.map_into_boxed_body())
        })
    }
}
