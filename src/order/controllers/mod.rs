use actix_web::{get, HttpResponse, post, Responder};
use actix_web::web::Data;
use crate::order::service::OrderService;
use crate::shared::error_handler::ApiError;
use crate::shared::responses::MessageResponse;
use crate::shared::service::token_data::JwtMiddleware;

#[get("/order")]
pub async fn get_user_orders(service: Data<OrderService>, payload: JwtMiddleware)
                             -> Result<impl Responder, ApiError> {
    let result = service.user_order_list(&payload.user_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/order")]
pub async fn order_cart(service: Data<OrderService>, payload: JwtMiddleware)
                        -> Result<impl Responder, ApiError> {
    let result = service.order_cart(&payload.user_id).await?;
    Ok(HttpResponse::Created().json(MessageResponse::new(result)))
}