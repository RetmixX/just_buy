use crate::cart::repository::cart_model::CartUser;
use crate::DBPool;
use crate::shared::error_handler::{ApiError, ErrorType};

pub struct CartData {
    db: DBPool,
}

impl CartData {
    pub fn new(db: DBPool) -> Self {
        Self { db }
    }
}


impl CartData {
    pub async fn show_user_cart(&self, id_user: &i32) -> Result<Vec<CartUser>, ApiError> {
        let query = r#"select carts_products.id as cart_id, products.id as product_id,
                                name, description, price from carts
                                join carts_products on carts.id = carts_products.cart_id
                                join products on carts_products.product_id
                                 = products.id where user_id = $1 and ordered = false"#;

        let data = sqlx::query_as::<_, CartUser>(query)
            .bind(id_user).fetch_all(&self.db).await?;

        Ok(data)
    }

    pub async fn add_product_to_cart(&self, id_user: &i32, product_id: &i32)
                                     -> Result<(), ApiError> {
        sqlx::query(r#"select id from products where id = $1"#).bind(product_id)
            .fetch_one(&self.db).await?;

        let query = r#"insert into carts_products(cart_id, product_id)
                    select carts.id, $1 from carts where user_id = $2 and ordered = false"#;

        sqlx::query(query).bind(product_id)
            .bind(id_user).execute(&self.db).await?;

        Ok(())
    }

    pub async fn remove_product_from_cart(&self, id_user: &i32, cart_id: &i32) -> Result<(), ApiError> {
        let query = r#"delete from carts_products using carts where carts_products.id = $1
                         and carts_products.cart_id = (
                            select carts.id from carts
                            where carts.user_id = $2 and carts.ordered = false
                         ) returning carts_products.id"#;

        let result = sqlx::query::<_>(query).bind(cart_id)
            .bind(id_user).fetch_optional(&self.db)
            .await?;

        if let None = result {
            return Err(ApiError {
                message: "Forbidden for you".to_string(),
                type_error: ErrorType::Forbidden,
            });
        }

        Ok(())
    }
}