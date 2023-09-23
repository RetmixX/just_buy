use sqlx::FromRow;
use crate::product::dto::ProductDto;

#[derive(FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
}

impl From<ProductDto> for Product {
    fn from(value: ProductDto) -> Self {
        Self {
            ..value.into()
        }
    }
}

