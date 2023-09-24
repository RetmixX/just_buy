use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct OrderUserDto {
    pub id: i32,
    pub products: Vec<i32>,
    pub price: i32,
}