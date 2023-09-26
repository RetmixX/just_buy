use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::user::controllers::registration,
        crate::user::controllers::login,
        crate::user::controllers::logout,
        crate::product::controllers::get_product,
        crate::product::controllers::index_product,
        crate::product::controllers::update_product,
        crate::product::controllers::create_product,
        crate::product::controllers::delete_product,
        crate::cart::controllers::add_product_to_cart,
        crate::cart::controllers::delete_product_from_cart,
        crate::cart::controllers::show_cart,
        crate::order::controllers::get_user_orders,
        crate::order::controllers::order_cart
    ),
    components(
        schemas(
            crate::order::dto::OrderUserDto,
            crate::product::dto::ProductDto,
            crate::product::dto::UpsertProductDto,
            crate::cart::dto::CartUserDto,
            crate::user::dto::LoginUser,
            crate::user::dto::RegisterUser,
            crate::user::dto::UserToken,
            crate::shared::responses::MessageResponse,
            crate::shared::error_handler::ApiErrorResponse,
            crate::shared::utils::config_json_validation::JsonErrorPayload,
            crate::shared::utils::config_json_validation::ErrorValidationInfo
        )
    ),
    modifiers(&SecurityAddon)
)]
pub struct SwaggerDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}