use crate::DBPool;
use crate::shared::error_handler::{ApiError, ErrorType};
use crate::user::dto::{LoginUser, RegisterUser, UserToken};
use crate::user::repository::user_data::UserData;
use crate::user::utils::password_utils::{hash_password, verify_password};
use crate::user::utils::token::token_utils::generate_token;

pub struct UserService {
    pub user_data: UserData,
}

impl UserService {
    pub fn new(db: DBPool) -> Self {
        Self {
            user_data: UserData::new(db),
        }
    }
}

impl UserService {
    pub async fn register_user(&self, data: &RegisterUser) -> Result<UserToken, ApiError> {
        let hash_password = hash_password(&data.password.clone().unwrap());
        let new_user = self.user_data.create_user(data, hash_password).await?;

        let token = generate_token(&new_user);
        self.user_data.update_token(&new_user.id, &token).await?;

        Ok(UserToken { token })
    }

    pub async fn login_user(&self, data: &LoginUser) -> Result<UserToken, ApiError> {
        let found_user =
            self.user_data.user_by_email(data.email.clone().unwrap()).await?;


        match verify_password(
            &data.password.clone().unwrap(),
            &found_user.password.clone(),
        ) {
            true => {
                let token = generate_token(&found_user);
                self.user_data.update_token(&found_user.id, &token).await?;
                Ok(UserToken { token })
            }

            false => {
                return Err(ApiError {
                    message: "Wrong password or email".to_string(),
                    type_error: ErrorType::Unauthorized,
                });
            }
        }
    }

    pub async fn logout(&self, id: &i32) -> Result<(), ApiError> {
        self.user_data.update_token(id, &"".to_string()).await?;
        Ok(())
    }

    pub async fn email_validation(&self, email: String) -> Result<(), ApiError> {
        match self.user_data.user_by_email(email).await {
            Ok(_) => Err(ApiError {
                message: "Wrong email".to_string(),
                type_error: ErrorType::ValidationUnique,
            }),
            Err(_) => Ok(())
        }
    }
}