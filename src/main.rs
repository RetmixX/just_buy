mod utils;
mod models;
mod repository;
mod schema;

use std::env;
use actix_web::{App, delete, get, HttpResponse, HttpServer, patch, post, Responder, web};
use actix_web::web::{Data, Path};
use dotenv::dotenv;
use sqlx::{PgPool, Postgres};
use crate::models::models_data::{NewProduct, Product, UpdateProduct};
use crate::repository::database::{Database, DbError};

type DBPool = sqlx::Pool<Postgres>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DB URL NOT SET");

    let pool = PgPool::connect(&db_url).await
        .expect("Cannot create connect to DB");
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db = Database::new();
    let data = web::Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(config)
            .wrap(actix_web::middleware::Logger::default())
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/products")]
async fn index_products(db: web::Data<Database>) -> impl Responder {
    match db.find_all_products() {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().body("Server Error")
    }
}

#[get("/products/{id_product}")]
async fn show_product(db: web::Data<Database>, product_id: web::Path<i32>) -> impl Responder {
    match db.find_by_id_product(&product_id) {
        None => HttpResponse::NotFound().body(format!("Product by id - {} not found", product_id)),
        Some(product) => HttpResponse::Ok().json(product)
    }
}

#[post("/products")]
async fn create_product(db: web::Data<Database>, new_product: web::Json<NewProduct>)
                        -> impl Responder {
    match db.create_product(new_product.into_inner()) {
        Ok(product) => HttpResponse::Created().json(product),
        Err(_) => HttpResponse::InternalServerError().body("Server Error")
    }
}

#[patch("/products/{id_product}")]
async fn update_product(db: web::Data<Database>, update_product: web::Json<UpdateProduct>,
                        id_product: web::Path<i32>) -> impl Responder {
    match db.update_product(&id_product, update_product.into_inner()) {
        None => HttpResponse::NotFound()
            .body(format!("Product by id - {}, not found", id_product)),

        Some(product) => HttpResponse::Ok().json(product)
    }
}

#[delete("/products/{id_product}")]
async fn delete_product(db: web::Data<Database>, id_product: web::Path<i32>) -> impl Responder {
    match db.delete_product(&id_product) {
        None => HttpResponse::NotFound().body(format!("Product by id - {} not found", &id_product)),
        Some(product) => HttpResponse::Ok().body(format!("Product by id - {} deleted", product))
    }
}

#[get("/test")]
async fn get_test(db: Data<Database>) -> impl Responder{
    db.get_cart_with_products(1);

    HttpResponse::Ok()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(index_products)
            .service(create_product)
            .service(show_product)
            .service(delete_product)
            .service(update_product)
            .service(get_test)
    );
}