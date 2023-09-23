use crate::shared::error_handler::{ApiError, ErrorType};

pub async fn check_role(real: String, required: String) -> Result<(), ApiError> {
    match required == real {
        true => Ok(()),
        false => Err(ApiError {
            message: "Forbidden for you".to_string(),
            type_error: ErrorType::Grand,
        })
    }
}