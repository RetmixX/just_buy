use serde::Serialize;

#[derive(Serialize)]
pub struct OrderUserDto {
    pub id: i32,
    pub products: Vec<i32>,
    pub price: i32,
}