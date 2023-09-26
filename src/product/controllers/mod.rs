use actix_web::{delete, get, HttpResponse, patch, post, Responder};
use actix_web::web::{Data, Path};
use actix_web_validator::Json;
use crate::product::dto::{UpsertProductDto};
use crate::product::service::ProductService;
use crate::shared::error_handler::ApiError;
use crate::shared::responses::MessageResponse;
use crate::shared::service::token_data::JwtMiddleware;
use crate::shared::utils::grant_validator::check_role;

#[utoipa::path(
    get,
    path = "/api/products",
    tag = "Эндпоинт для просмотра списка товаров",
    responses(
        (
            status = 200,
            description = "Позволяет всем пользователям посмотреть все товары из бд",
            body = ProductDto
        ),
        (
            status = 500,
            body = ApiErrorResponse
        )
    )
)]
#[get("/products")]
pub async fn index_product(service: Data<ProductService>) -> Result<impl Responder, ApiError> {
    let response = HttpResponse::Ok().json(service.find_all_products().await?);
    Ok(response)
}

#[utoipa::path(
    get,
    path = "/api/products/{id}",
    tag = "Эндпоинт для просмотра товара",
    responses(
        (
            status = 200, description = "Позволяет всем пользователям посмотреть товар по ид",
            body = ProductDto
        ),
        (
            status = 404,
            description = "Ошибка возникает, когда товара с таким ид нету",
            body = ApiErrorResponse
        ),
        (
            status = 500,
            body = ApiErrorResponse
        )
    )
)]
#[get("/products/{id}")]
pub async fn get_product(service: Data<ProductService>, id: Path<i32>) -> Result<impl Responder, ApiError> {
    let response = HttpResponse::Ok().json(service.find_product(&id).await?);
    Ok(response)
}

#[utoipa::path(
    post,
    path = "/api/products",
    tag = "Эндпоинт для создание товара",
    responses(
        (
            status = 201,
            description = "Позволяет администраторам создать товар",
            body = ProductDto
        ),
        (
            status = 401,
            description = "Ошибка если нету токена/не валидный или пользователь не админ",
            body = ApiErrorResponse
        ),
        (
            status = 422,
            description = "Поля были заполнены не верно",
            body = JsonErrorPayload
        ),
        (
        status = 500,
        body = ApiErrorResponse
        ),
    ),
    security(
        ("token" = [])
    )
)]
#[post("/products")]
pub async fn create_product(service: Data<ProductService>,
                            data: Json<UpsertProductDto>, payload: JwtMiddleware)
                            -> Result<impl Responder, ApiError> {
    check_role(payload.role, "ROLE_ADMIN".to_string()).await?;
    let result = service.create_product(&data).await?;
    Ok(HttpResponse::Created().json(result))
}

#[utoipa::path(
    patch,
    path = "/api/products/{id}",
    tag = "Эндпоинт для обновления товара",
    responses(
        (
            status = 200,
            description = "Позволяет администратору обновить товар по ид",
            body = ProductDto
        ),
        (
            status = 401,
            description = "Ошибка если нету токена/не валидный или пользователь не админ",
            body = ApiErrorResponse
        ),
        (
            status = 422,
            description = "Поля были заполнены не верно",
            body = JsonErrorPayload
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
#[patch("/products/{id}")]
pub async fn update_product(service: Data<ProductService>, id: Path<i32>,
                            data: Json<UpsertProductDto>, payload: JwtMiddleware)
                            -> Result<impl Responder, ApiError> {
    check_role(payload.role, "ROLE_ADMIN".to_string()).await?;
    let result = service.update_product(&id, &data).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    delete,
    path = "/api/products/{id}",
    tag = "Эндпоинт для удаления товара",
    responses(
    (
        status = 200,
        description = "Позволяет администратору удалять товар по ид",
        body = ProductDto
    ),
    (
        status = 401,
        description = "Ошибка если нету токена/не валидный или пользователь не админ",
        body = ApiErrorResponse
    ),
    (
        status = 422,
        description = "Поля были заполнены не верно",
        body = JsonErrorPayload
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
#[delete("/products/{id}")]
pub async fn delete_product(service: Data<ProductService>, id: Path<i32>,
                            payload: JwtMiddleware) -> Result<impl Responder, ApiError> {
    check_role(payload.role, "ROLE_ADMIN".to_string()).await?;
    service.delete_product(&id).await?;
    Ok(HttpResponse::Ok().json(MessageResponse::new("Product deleted".to_string())))
}