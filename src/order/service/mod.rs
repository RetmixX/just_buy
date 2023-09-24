use std::collections::HashMap;
use crate::order::dto::OrderUserDto;
use crate::order::repository::order_data::OrderData;
use crate::shared::db::DbPool;
use crate::shared::error_handler::ApiError;

pub struct OrderService {
    pub order_data: OrderData,
}

impl OrderService {
    pub fn new(db: DbPool) -> Self {
        Self {
            order_data: OrderData::new(db)
        }
    }
}

impl OrderService {
    pub async fn user_order_list(&self, user_id: &i32) -> Result<Vec<OrderUserDto>, ApiError> {
        let data = self.order_data.user_order(user_id).await?;
        let mut groups_orders = HashMap::new();

        for cart in data {
            let entry = groups_orders.entry(cart.id).or_insert_with(||
                OrderUserDto {
                    id: cart.id,
                    products: vec![],
                    price: 0,
                });
            entry.products.push(cart.product_id);
            entry.price += cart.price;
        }

        let cart_user = groups_orders.into_values().collect::<Vec<OrderUserDto>>();
        Ok(cart_user)
    }

    pub async fn order_cart(&self, user_id: &i32) -> Result<String, ApiError> {
        let result = self.order_data.order_cart(user_id).await?;

        Ok(format!("Order in process {}", result))
    }
}