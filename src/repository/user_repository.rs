use diesel::prelude::*;
use crate::models::models_data::{User, Cart, CartProduct, Product};
use crate::repository::database::{Database, DbError};
use crate::schema::*;
use crate::schema::carts::user_id;

impl Database {
    pub fn get_cart_with_products(&self, id_user: i32) {

    }
}