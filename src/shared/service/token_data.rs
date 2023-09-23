use serde::{Deserialize, Serialize};

pub struct JwtMiddleware {
    pub user_id: i32,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub iat: usize,
    pub exp: usize,
    pub user_id: String,
    pub role: String
}