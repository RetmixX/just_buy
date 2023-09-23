use actix_web::{get, HttpResponse, post, Responder};
use actix_web::web::Data;
use actix_web_validator::Json;
use crate::shared::error_handler::ApiError;
use crate::shared::responses::MessageResponse;
use crate::shared::service::token_data::JwtMiddleware;
use crate::user::dto::{LoginUser, RegisterUser};
use crate::user::service::user_service::UserService;

#[post("/signup")]
pub async fn registration(service: Data<UserService>, data: Json<RegisterUser>)
                          -> Result<impl Responder, ApiError> {
    service.email_validation(data.email.clone().unwrap()).await?;
    let result = service.register_user(&data).await?;

    Ok(HttpResponse::Created().json(result))
}

#[post("/login")]
pub async fn login(service: Data<UserService>, data: Json<LoginUser>) -> Result<impl Responder, ApiError> {
    let result = service.login_user(&data).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/logout")]
pub async fn logout(service: Data<UserService>, payload: JwtMiddleware) -> Result<impl Responder, ApiError> {
    service.logout(&payload.user_id).await?;
    Ok(HttpResponse::Ok().json(MessageResponse::new("Exit".to_string())))
}