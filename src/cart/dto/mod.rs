use serde::Serialize;
use utoipa::ToSchema;
use crate::cart::repository::cart_model::CartUser;

#[derive(Serialize, ToSchema)]
pub struct CartUserDto {
    pub id: i32,
    pub product_id: i32,
    pub name: String,
    pub description: String,
    pub price: i32,
}

impl CartUserDto {
    pub fn from(model: &CartUser) -> Self {
        Self {
            id: model.cart_id,
            product_id: model.product_id,
            name: model.name.clone(),
            description: model.description.clone(),
            price: model.price,
        }
    }
}