use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use crate::shared::utils::load_configs::ConfigApp;
use crate::user::repository::model::User;
use jsonwebtoken::errors::Result;
use crate::shared::service::token_data::TokenClaims;

pub fn generate_token(data: &User) -> String {
    let config = ConfigApp::new();
    let secret = config.token.secret.as_bytes();
    let exp = config.token.exp;

    let now = Utc::now();

    let payload = TokenClaims {
        iat: now.timestamp() as usize,
        exp: (now + Duration::minutes(exp as i64)).timestamp() as usize,
        user_id: data.id.to_string(),
        role: data.role.clone(),
    };

    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret),
    ).unwrap()
}

pub fn validate_token(token: String) -> Result<TokenData<TokenClaims>> {
    let config = ConfigApp::new();
    let secret = config.token.secret;

    decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}
