use actix_web::{get, HttpResponse, post, Responder};
use actix_web::web::Data;
use actix_web_validator::Json;
use crate::shared::error_handler::ApiError;
use crate::shared::responses::MessageResponse;
use crate::shared::service::token_data::JwtMiddleware;
use crate::user::dto::{LoginUser, RegisterUser};
use crate::user::service::user_service::UserService;

#[utoipa::path(
    post,
    path = "/api/signup",
    tag = "Эндпоинт для регистрации",
    responses(
        (
            status = 201,
            description = "Регистрация пользователя в базе данных по введеным полям,\
                при успешной регистрации возвращается поле с токеном",
            body = UserToken
        ),
        (
            status = 422,
            description = "Поля были заполнены не верно",
            body = JsonErrorPayload
        ),
        (
            status = 422,
            description = "Поле почта не уникально",
            body = ApiErrorResponse
        ),
        (
            status = 500,
            body = ApiErrorResponse
        )
    ),

)]
#[post("/signup")]
pub async fn registration(service: Data<UserService>, data: Json<RegisterUser>)
                          -> Result<impl Responder, ApiError> {
    service.email_validation(data.email.clone().unwrap()).await?;
    let result = service.register_user(&data).await?;

    Ok(HttpResponse::Created().json(result))
}

#[utoipa::path(
    get,
    path = "/api/login",
    tag = "Эндпоинт для авторизации пользователя",
    responses(
        (
            status = 200,
            description = "Для авторизации существующего пользователя, в случае успешной\
             авторизации возвращается поле с токеном",
            body = UserToken
        ),
        (
            status = 401,
            description = "Почта или пароль не верны",
            body = ApiErrorResponse
        ),
        (
            status = 500,
            body = ApiErrorResponse
        )
    )
)]
#[post("/login")]
pub async fn login(service: Data<UserService>, data: Json<LoginUser>) -> Result<impl Responder, ApiError> {
    let result = service.login_user(&data).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/api/logout",
    tag = "Эндпоинт выхода из аккуанта",
    responses(
    (
        status = 200,
        description = "Очищает поле логина у пользователя в бд",
        body = ProductDto
    ),
    (
        status = 401,
        description = "Ошибка если нету токена/не валидный",
        body = ApiErrorResponse
    ),
    (
        status = 500,
        body = ApiErrorResponse
    )
    ),
    security(
    ("token" = [])
    )
)]
#[get("/logout")]
pub async fn logout(service: Data<UserService>, payload: JwtMiddleware) -> Result<impl Responder, ApiError> {
    service.logout(&payload.user_id).await?;
    Ok(HttpResponse::Ok().json(MessageResponse::new("Exit".to_string())))
}