use sqlx::{FromRow};
use crate::DBPool;
use crate::order::repository::order_model::UserOrder;
use crate::shared::error_handler::{ApiError, ErrorType};

pub struct OrderData {
    db: DBPool,
}

impl OrderData {
    pub fn new(db: DBPool) -> Self {
        Self { db }
    }
}


impl OrderData {
    pub async fn order_cart(&self, user_id: &i32) -> Result<i32, ApiError> {
        let query = r#"update carts set ordered = true where carts.id = (select carts.id
                         from carts where user_id = $1 and ordered = false
                         and 0 < (select count(*)
                         from carts_products
                         join carts on carts_products.cart_id = carts.id
                         where user_id = $2
                         and ordered = false) limit 1) returning carts.id as id"#;

        let result = sqlx::query_as::<_, IdOrder>(query).bind(user_id)
            .bind(user_id)
            .fetch_optional(&self.db).await?;

        if result.is_none() {
            return Err(ApiError::new("Cart is empty".to_string(),
                                     ErrorType::Validation));
        }

        sqlx::query("insert into carts(user_id) values($1)").bind(user_id)
            .execute(&self.db).await?;

        Ok(result.unwrap().id)
    }


    pub async fn user_order(&self, user_id: &i32) -> Result<Vec<UserOrder>, ApiError> {
        let query = r#"select carts.id as id, product_id, products.price as price from carts
                           join carts_products on carts.id = carts_products.cart_id
                           join products on carts_products.product_id = products.id
                           where user_id = $1 and ordered = true"#;

        let result = sqlx::query_as::<_, UserOrder>(query)
            .bind(user_id)
            .fetch_all(&self.db).await?;

        Ok(result)
    }
}

#[derive(FromRow)]
struct IdOrder{
    id: i32
}