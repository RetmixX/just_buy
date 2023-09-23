use sqlx::FromRow;

#[derive(FromRow)]
pub struct CartUser {
    pub cart_id: i32,
    pub product_id: i32,
    pub name: String,
    pub description: String,
    pub price: i32
}