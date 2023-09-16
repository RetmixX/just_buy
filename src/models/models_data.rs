use serde::{Serialize, Deserialize};
use crate::schema::{products, users, carts, carts_products};
use diesel::prelude::*;

#[derive(Debug, Identifiable, Selectable, Queryable)]
#[diesel(table_name=users)]
#[has_many(Cart)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub patronymic: Option<String>,
    pub password: String
}

#[derive(Debug, Queryable, Selectable, PartialEq, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
}

#[derive(Associations, Debug, Queryable, Selectable, Identifiable, PartialEq)]
#[diesel(belongs_to(User))]
#[diesel(table_name=carts)]
#[has_many(CartProduct)]
pub struct Cart{
    pub id: i32,
    pub user_id: i32,
    pub ordered: bool
}

#[derive(Debug, Selectable, Associations, PartialEq, Queryable, Identifiable)]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
#[diesel(primary_key(cart_id, product_id))]
#[table_name="carts_products"]
pub struct CartProduct {
    pub id: i32,
    pub cart_id: i32,
    pub product_id: i32
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "products"]
pub struct NewProduct{
    pub name: String,
    pub description: Option<String>,
    pub price: i32
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "products"]
pub struct UpdateProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: i32
}