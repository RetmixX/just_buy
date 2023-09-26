use actix_web::{HttpResponse, Responder};
use crate::shared::responses::MessageResponse;

pub async fn default_not_found() -> impl Responder {
    HttpResponse::NotFound()
        .json(MessageResponse::new("Endpoint not found".to_string()))
}