use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub patronymic: Option<String>,
    pub password: String,
    pub token: Option<String>,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims{
    pub user_id: String,
    pub role: String,
    pub iat: usize,
    pub exp: usize
}