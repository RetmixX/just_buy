use actix_web::{error, HttpResponse};
use actix_web_validator::{JsonConfig, Error};
use serde::Serialize;
use utoipa::ToSchema;
use validator::ValidationErrors;

#[derive(Serialize, ToSchema)]
pub struct JsonErrorPayload {
    message: String,
    fields: Vec<ErrorValidationInfo>,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorValidationInfo {
    field: String,
    info: Vec<String>,
}

impl From<&ValidationErrors> for JsonErrorPayload {
    fn from(value: &ValidationErrors) -> Self {
        JsonErrorPayload {
            message: "Validation error".to_owned(),
            fields: value.field_errors().iter()
                .map(|(field, error_info)|
                    ErrorValidationInfo {
                        field: field.to_string(),
                        info: error_info.to_vec().clone().iter().map(|t|
                            t.message.clone().iter().map(|message| message.clone().to_string())
                                .collect()).collect(),
                    }
                ).collect(),
        }
    }
}

pub fn config_json_validation() -> JsonConfig {
    JsonConfig::default().error_handler(
        |err, _| {
            let json_error = match &err {
                Error::Validate(error) => JsonErrorPayload::from(error),
                _ => JsonErrorPayload { message: err.to_string(), fields: Vec::new() }
            };

            error::InternalError::from_response(err,
                                                HttpResponse::UnprocessableEntity()
                                                    .json(json_error)).into()
        }
    )
}