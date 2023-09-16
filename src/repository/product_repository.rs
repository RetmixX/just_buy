use diesel::prelude::*;
use crate::models::models_data::{NewProduct, Product, UpdateProduct};
use crate::repository::database::{Database, DbError};
use crate::schema::products::dsl::*;

impl Database {
    pub fn find_all_products(&self) -> Result<Vec<Product>, DbError> {
        let result =
            products.load::<Product>(&mut self.pool_connection.get().unwrap())?;
        Ok(result)
    }

    pub fn find_by_id_product(&self, id_product: &i32) -> Option<Product> {
        products.find(id_product)
            .get_result::<Product>(&mut self.pool_connection.get().unwrap()).ok()
    }

    pub fn create_product(&self, new_product: NewProduct) -> Result<Product, DbError> {
        let result = diesel::insert_into(products)
            .values(new_product).returning(Product::as_returning())
            .get_result(&mut self.pool_connection.get().unwrap())?;
        Ok(result)
    }

    pub fn delete_product(&self, id_product: &i32) -> Option<usize> {
        diesel::delete(products.find(id_product))
            .execute(&mut self.pool_connection.get().unwrap()).ok()
    }

    pub fn update_product(&self, id_product: &i32, update_product: UpdateProduct)
                          -> Option<Product> {

        diesel::update(products.find(id_product)).set(update_product)
            .get_result::<Product>(&mut self.pool_connection.get().unwrap()).ok()
    }
}