use actix_web::{get, HttpResponse, post, Responder};
use actix_web::web::Data;
use crate::order::service::OrderService;
use crate::shared::error_handler::ApiError;
use crate::shared::responses::MessageResponse;
use crate::shared::service::token_data::JwtMiddleware;

#[utoipa::path(
    get,
    path = "/api/order",
    tag = "Эндпоинт для просмотра заказов текущего пользователя",
    responses(
        (
            status = 200, description = "Просмотр всех заказов текущего пользова еля",
            body = OrderUserDto
        ),
        (
            status = 401,
            body = ApiErrorResponse
        ),
        (
            status = 500,
            body = ApiErrorResponse
        )
    )
)]
#[get("/order")]
pub async fn get_user_orders(service: Data<OrderService>, payload: JwtMiddleware)
                             -> Result<impl Responder, ApiError> {
    let result = service.user_order_list(&payload.user_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    post,
    path = "/api/order",
    tag = "Эндпоинт для оформления заказа",
    responses(
        (
        status = 200,
        description = "Оформление заказа для текущей корзниы пользователя",
        body = MessageResponse
        ),
        (
            status = 422,
            description = "Данная ошибка появляется, когда пользователь пытается оформить заказ с пустой корзиной",
            body = ApiErrorResponse
        ),
        (
            status = 401,
            body = ApiErrorResponse
        ),
        (
            status = 500,
            body = ApiErrorResponse
        )
    )
)]
#[post("/order")]
pub async fn order_cart(service: Data<OrderService>, payload: JwtMiddleware)
                        -> Result<impl Responder, ApiError> {
    let result = service.order_cart(&payload.user_id).await?;
    Ok(HttpResponse::Created().json(MessageResponse::new(result)))
}