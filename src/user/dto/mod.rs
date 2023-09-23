use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterUser {
    #[validate(required(message = "Email is required"),
    email(message = "Invalid format"))]
    pub email: Option<String>,
    #[validate(required(message = "Password is required"))]
    pub password: Option<String>,
    #[validate(required(message = "Name is required"),
    length(min = 1, max = 100, message = "Length: min:=1, max:=100"))]
    pub name: Option<String>,
    #[validate(required(message = "Surname is required"),
    length(min = 1, max = 100, message = "Length: min:=1, max:=100"))]
    pub surname: Option<String>,
    #[validate(required(message = "Patronymic is required"),
    length(min = 1, max = 100, message = "Length: min:=1, max:=100"))]
    pub patronymic: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct LoginUser {
    #[validate(required(message = "Email is required"))]
    pub email: Option<String>,
    #[validate(required(message = "Password is required"))]
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct UserToken {
    pub token: String,
}