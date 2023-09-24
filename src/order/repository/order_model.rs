use sqlx::FromRow;

#[derive(FromRow, Clone)]
pub struct UserOrder {
    pub id: i32,
    pub product_id: i32,
    pub price: i32,
}

#[derive(FromRow)]
pub struct OrderId{
    pub id_order: i32
}