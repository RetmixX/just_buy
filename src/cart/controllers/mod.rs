use actix_web::{delete, get, HttpResponse, post, Responder};
use actix_web::web::{Data, Path};
use crate::cart::service::CartService;
use crate::shared::error_handler::ApiError;
use crate::shared::responses::MessageResponse;
use crate::shared::service::token_data::JwtMiddleware;

#[get("/cart")]
pub async fn show_cart(service: Data<CartService>, payload: JwtMiddleware)
                       -> Result<impl Responder, ApiError> {
    let result =
        service.user_cart(&payload.user_id).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[post("/cart/{id_product}")]
pub async fn add_product_to_cart(service: Data<CartService>,
                                 product_id: Path<i32>,
                                 payload: JwtMiddleware)
                                 -> Result<impl Responder, ApiError> {
    service.add_product_to_cart(&payload.user_id, &product_id).await?;

    Ok(HttpResponse::Created()
        .json(MessageResponse::new("Product added".to_string())))
}

#[delete("/cart/{id}")]
pub async fn delete_product_from_cart(service: Data<CartService>,
                                      position: Path<i32>,
                                      payload: JwtMiddleware)
                                      -> Result<impl Responder, ApiError> {
    service.remove_product_from_user(&payload.user_id, &position).await?;

    Ok(HttpResponse::Ok()
        .json(MessageResponse::new("Product removed from cart".to_string())))
}