use actix_web::{delete, get, HttpResponse, patch, post, Responder};
use actix_web::web::{Data, Path};
use actix_web_validator::Json;
use crate::product::dto::{UpsertProductDto};
use crate::product::service::ProductService;
use crate::shared::error_handler::ApiError;
use crate::shared::responses::MessageResponse;
use crate::shared::service::token_data::JwtMiddleware;
use crate::shared::utils::grant_validator::check_role;

#[get("/products")]
pub async fn index_product(service: Data<ProductService>) -> Result<impl Responder, ApiError> {
    let response = HttpResponse::Ok().json(service.find_all_products().await?);
    Ok(response)
}

#[get("/products/{id}")]
pub async fn get_product(service: Data<ProductService>, id: Path<i32>) -> Result<impl Responder, ApiError> {
    let response = HttpResponse::Ok().json(service.find_product(&id).await?);
    Ok(response)
}

#[post("/products")]
pub async fn create_product(service: Data<ProductService>,
                            data: Json<UpsertProductDto>, payload: JwtMiddleware)
                            -> Result<impl Responder, ApiError> {
    check_role(payload.role, "ROLE_ADMIN".to_string()).await?;
    let result = service.create_product(&data).await?;
    Ok(HttpResponse::Created().json(result))
}

#[patch("/products/{id}")]
pub async fn update_product(service: Data<ProductService>, id: Path<i32>,
                            data: Json<UpsertProductDto>, payload: JwtMiddleware)
                            -> Result<impl Responder, ApiError> {
    check_role(payload.role, "ROLE_ADMIN".to_string()).await?;
    let result = service.update_product(&id, &data).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[delete("/products/{id}")]
pub async fn delete_product(service: Data<ProductService>, id: Path<i32>,
                            payload: JwtMiddleware) -> Result<impl Responder, ApiError> {
    check_role(payload.role, "ROLE_ADMIN".to_string()).await?;
    service.delete_product(&id).await?;
    Ok(HttpResponse::Ok().json(MessageResponse::new("Product deleted".to_string())))
}