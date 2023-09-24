use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TokenClaims {
    pub iat: usize,
    pub exp: usize,
    pub user_id: String,
    pub role: String
}

#[derive(Serialize, Deserialize)]
pub struct TokenByResponse{
    pub token: String,
    pub token_type: String
}