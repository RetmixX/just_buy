mod shared;
mod user;
mod product;
mod cart;
mod order;

use std::env;
use actix_files::Files;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use dotenv::dotenv;
use sqlx::{Postgres};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::cart::service::CartService;
use crate::order::service::OrderService;
use crate::product::service::ProductService;
use crate::shared::controllers::default_not_found;
use crate::shared::db::get_connection;
use crate::shared::service::token_service::TokenService;
use crate::shared::utils::config_json_validation::config_json_validation;
use crate::shared::utils::documentation::SwaggerDoc;
use crate::shared::utils::register_routers::config;
use crate::user::service::user_service::UserService;


type DBPool = sqlx::Pool<Postgres>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let db_pool = get_connection().await.clone().unwrap();
    sqlx::migrate!().run(&db_pool).await.expect("Cannot run migration");
    let (data_products,
        data_users,
        data_cart,
        token_data,
        order_data)
        = load_app_data(db_pool.clone());

    let json_config = config_json_validation();
    let swagger = SwaggerDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(token_data.clone())
            .app_data(data_products.clone())
            .app_data(data_users.clone())
            .app_data(data_cart.clone())
            .app_data(order_data.clone())
            .app_data(json_config.clone())
            .configure(config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", swagger.clone())
            )
            .default_service(web::route().to(default_not_found))
            .service(Files::new("/images", "static/images/")
                .show_files_listing())
            .service(Files::new("/", "./static/root")
                .index_file("index.html"))
            .wrap(actix_web::middleware::Logger::default())
    }).bind(("0.0.0.0", 8000))?
        .run()
        .await
}

fn load_app_data(connection: DBPool)
                 -> (Data<ProductService>, Data<UserService>,
                     Data<CartService>, Data<TokenService>, Data<OrderService>) {
    let data_products = Data::new(ProductService::new(connection.clone()));
    let data_users = Data::new(UserService::new(connection.clone()));
    let data_cart = Data::new(CartService::new(connection.clone()));
    let token_data = Data::new(TokenService::new(connection.clone()));
    let order_data = Data::new(OrderService::new(connection.clone()));

    (data_products, data_users, data_cart, token_data, order_data)
}