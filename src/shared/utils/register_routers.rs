use actix_web::web;
use crate::cart::controllers::{add_product_to_cart, delete_product_from_cart, show_cart};
use crate::product::controllers::{get_product,
                                  index_product,
                                  update_product,
                                  create_product,
                                  delete_product};

use crate::user::controllers::{login, logout, registration};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(index_product)
            .service(get_product)
            .service(update_product)
            .service(create_product)
            .service(delete_product)
            .service(registration)
            .service(login)
            .service(logout)
            .service(show_cart)
            .service(add_product_to_cart)
            .service(delete_product_from_cart)
    );
}