use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::product::repository::product_model::Product;

#[derive(Serialize)]
pub struct ProductDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
}

#[derive(Deserialize, Validate)]
pub struct UpsertProductDto {
    #[validate(length(min = 1, max = 100, message = "Length: min:= 10, max:=100"),
    required(message = "Name is required"))]
    pub name: Option<String>,
    #[validate(length(max = 100, message = "Description max 100 symbols"))]
    pub description: Option<String>,
    #[validate(required(message = "Price is required"))]
    pub price: Option<i32>,
}


impl ProductDto {
    pub fn from(value: &Product) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            description: value.description.clone(),
            price: value.price,
        }
    }
}