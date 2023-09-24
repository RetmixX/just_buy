use actix_web::{delete, get, HttpResponse, post, Responder};
use actix_web::web::{Data, Path};
use crate::cart::service::CartService;
use crate::shared::error_handler::ApiError;
use crate::shared::responses::MessageResponse;
use crate::shared::service::token_data::JwtMiddleware;

#[utoipa::path(
    post,
    path = "/api/cart/{id_product}",
    tag = "Эндпоинт для добавление товара в корзину",
    responses(
        (status = 201, description = "Позволяет добавить продукт по ид в корзину текущего пользователя",
        body = MessageResponse)
    ),
    security(
        ("token" = [])
    )
)]
#[post("/cart/{id_product}")]
pub async fn add_product_to_cart(service: Data<CartService>,
                                 product_id: Path<i32>,
                                 payload: JwtMiddleware)
                                 -> Result<impl Responder, ApiError> {
    service.add_product_to_cart(&payload.user_id, &product_id).await?;

    Ok(HttpResponse::Created()
        .json(MessageResponse::new("Product added".to_string())))
}

#[utoipa::path(
    get,
    path = "/api/cart",
    tag = "Эндпоинт для просмотра корзины",
    responses(
        (
            status = 200,
            description = "Дает возможность посмотреть корзину текущего пользователя",
            body = CartUserDto
        ),
        (
            status = 401,
            body = ApiErrorResponse
        ),
        (
            status = 500,
            body = ApiErrorResponse
        )
    ),
    security(
    ("token" = [])
    )
)]
#[get("/cart")]
pub async fn show_cart(service: Data<CartService>, payload: JwtMiddleware)
                       -> Result<impl Responder, ApiError> {
    let result =
        service.user_cart(&payload.user_id).await?;

    Ok(HttpResponse::Ok().json(result))
}


#[utoipa::path(
    delete,
    path = "/api/cart/{id}",
    tag = "Эндпоинт для удаление товара из корзины",
    responses(
        (
            status = 200,
            description = "Позволяет удалить товар из корзины по ид корзины текущему пользователю",
            body = MessageResponse
        ),
        (
            status = 404,
            description = "Возвращается информация об ошибке, если объект не был найден по id"
        ),
        (
            status = 401,
            body = ApiErrorResponse
        ),
        (
            status = 500,
            body = ApiErrorResponse
        )
    ),
    security(
        ("token" = [])
    )
)]
#[delete("/cart/{id}")]
pub async fn delete_product_from_cart(service: Data<CartService>,
                                      position: Path<i32>,
                                      payload: JwtMiddleware)
                                      -> Result<impl Responder, ApiError> {
    service.remove_product_from_user(&payload.user_id, &position).await?;

    Ok(HttpResponse::Ok()
        .json(MessageResponse::new("Product removed from cart".to_string())))
}