use std::future::{Future};
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::Data;
use crate::shared::error_handler::ApiError;
use crate::shared::service::token_data::JwtMiddleware;
use crate::shared::service::token_service::TokenService;

impl FromRequest for JwtMiddleware {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = HttpRequest::clone(req);
        Box::pin(async move {
            let token_service = request.app_data::<Data<TokenService>>().unwrap();
            let jwt_middleware = token_service.validate_token(&request).await?;

            Ok(jwt_middleware)
        })
    }
}