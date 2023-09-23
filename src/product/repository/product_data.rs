use std::string::ToString;
use crate::DBPool;
use crate::product::dto::UpsertProductDto;
use crate::product::repository::product_model::Product;
use crate::shared::error_handler::ApiError;

pub struct ProductData {
    db: DBPool,
}

impl ProductData {
    pub fn new(db: DBPool) -> Self {
        Self { db }
    }
}

impl ProductData {
    pub async fn all_products(&self) -> Result<Vec<Product>, ApiError> {
        let query = r#"select * from products"#;
        let result = sqlx::query_as::<_, Product>(query)
            .fetch_all(&self.db).await?;

        Ok(result)
    }

    pub async fn get_product(&self, id: &i32) -> Result<Product, ApiError> {
        let query = r#"select * from products where id = $1"#;

        let result = sqlx::query_as::<_, Product>(query)
            .bind(id)
            .fetch_one(&self.db).await?;

        Ok(result)
    }

    pub async fn update_product(&self, id: &i32, data: &UpsertProductDto) -> Result<Product, ApiError> {
        let query = r#"update products set name = $1, description = $2,
                  price = $3
                   where id = $4
                    returning *"#;

        let result = sqlx::query_as::<_, Product>(query)
            .bind(data.name.clone().unwrap())
            .bind(data.description.clone().unwrap_or("".to_string()))
            .bind(data.price.clone().unwrap())
            .bind(id).fetch_one(&self.db).await?;

        Ok(result)
    }

    pub async fn create_product(&self, data: &UpsertProductDto) -> Result<Product, ApiError> {
        let query = r#"insert into products(name, description, price)
             VALUES ($1, $2, $3) returning *"#;

        let result = sqlx::query_as::<_, Product>(query)
            .bind(data.name.clone().unwrap())
            .bind(data.description.clone().unwrap_or("".to_string()))
            .bind(data.price.clone().unwrap())
            .fetch_one(&self.db).await?;

        Ok(result)
    }

    pub async fn delete_product(&self, id: &i32) -> Result<(), ApiError> {
        let query = r#"delete from products where id = $1"#;

        sqlx::query(query).bind(id).execute(&self.db).await?;

        Ok(())
    }
}

