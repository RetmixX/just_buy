use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterUser {
    #[schema(example = "example@gmail.com", required = true)]
    #[validate(required(message = "Email is required"),
    email(message = "Invalid format"))]
    pub email: Option<String>,
    #[schema(example = "password", required = true)]
    #[validate(required(message = "Password is required"))]
    pub password: Option<String>,
    #[schema(example = "Test", required = true)]
    #[validate(required(message = "Name is required"),
    length(min = 1, max = 100, message = "Length: min:=1, max:=100"))]
    pub name: Option<String>,
    #[schema(example = "Test", required = true)]
    #[validate(required(message = "Surname is required"),
    length(min = 1, max = 100, message = "Length: min:=1, max:=100"))]
    pub surname: Option<String>,
    #[schema(example = "Test", required = true)]
    #[validate(required(message = "Patronymic is required"),
    length(min = 1, max = 100, message = "Length: min:=1, max:=100"))]
    pub patronymic: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginUser {
    #[schema(example = "example@gmail.com", required = true)]
    #[validate(required(message = "Email is required"))]
    pub email: Option<String>,
    #[schema(example = "password", required = true)]
    #[validate(required(message = "Password is required"))]
    pub password: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct UserToken {
    pub token: String,
}