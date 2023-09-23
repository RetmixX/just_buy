use actix_web::http::header::HeaderMap;
use actix_web::{http, HttpRequest};
use crate::DBPool;
use crate::shared::error_handler::{ApiError, ErrorType};
use crate::shared::service::token_data::{JwtMiddleware, TokenClaims};
use crate::user::utils::token::token_utils::validate_token;

pub struct TokenService {
    pub db: DBPool,
}

impl TokenService {
    pub fn new(db: DBPool) -> Self {
        Self { db }
    }
}

impl TokenService {
    pub async fn validate_token(&self, request: &HttpRequest) -> Result<JwtMiddleware, ApiError> {
        let token = &self.get_token_from_headers(request.headers())?;
        let claims = self.get_claims_from_token(token)?;

        let user_id = claims.user_id.parse::<i32>().clone().unwrap();

        self.check_exist_token_user(token, &user_id).await?;

        Ok(JwtMiddleware {
            user_id,
            role: claims.role,
        })
    }

    fn get_token_from_headers(&self, header_map: &HeaderMap) -> Result<String, ApiError> {
        let token = header_map.get(http::header::AUTHORIZATION)
            .map(|header|
                header.to_str().unwrap().split_at(7).1.to_string());

        if token.is_none() {
            return Err(ApiError::
            new("Token is empty".to_string(), ErrorType::Unauthorized));
        }

        Ok(token.unwrap())
    }

    fn get_claims_from_token(&self, taken_token: &String)
                             -> Result<TokenClaims, ApiError> {
        match validate_token(taken_token.clone()) {
            Ok(data) => Ok(data.claims),
            Err(_) => Err(ApiError::new(
                "Token invalid".to_string(),
                ErrorType::Forbidden,
            ))
        }
    }

    async fn check_exist_token_user(&self, token: &String, user_id: &i32) -> Result<(), ApiError> {
        let result =
            sqlx::query::<_>("select id from users where token = $1 and id = $2")
                .bind(token).bind(user_id).fetch_optional(&self.db).await?;

        if let None = result {
            return Err(ApiError::new(
                "Token invalid".to_string(),
                ErrorType::Forbidden
            ))
        }

        Ok(())
    }
}
