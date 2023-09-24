use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use chrono::{DateTime, Local};
use serde::Serialize;
use sqlx::{Error as SqlxError, Error};
use validator::ValidationError;

#[derive(Debug)]
pub enum ErrorType {
    DbError,
    NotFound,
    Forbidden,
    Unauthorized,
    Validation,
    Grand,
}

#[derive(Debug)]
pub struct ApiError {
    pub message: String,
    pub type_error: ErrorType,
}

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub message: String,
    pub info: String,
    pub date_time: DateTime<Local>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ApiError {
    pub fn new(message: String, type_error: ErrorType) -> Self {
        Self {
            message,
            type_error,
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self.type_error {
            ErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::NotFound => StatusCode::NOT_FOUND,
            ErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorType::Forbidden => StatusCode::FORBIDDEN,
            ErrorType::Validation => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorType::Grand => StatusCode::FORBIDDEN
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(ApiErrorResponse {
                message: "Error".to_string(),
                info: self.message.clone(),
                date_time: Local::now(),
            })
    }
}

impl From<SqlxError> for ApiError {
    fn from(value: SqlxError) -> Self {
        match value {
            Error::RowNotFound => Self {
                message: "Object not found".to_string(),
                type_error: ErrorType::NotFound,
            },
            _ => Self {
                message: value.to_string(),
                type_error: ErrorType::DbError,
            }
        }
    }
}

impl From<ValidationError> for ApiError {
    fn from(value: ValidationError) -> Self {
        Self {
            message: value.code.to_string(),
            type_error: ErrorType::Validation,
        }
    }
}

impl From<SqlxError> for ErrorType {
    fn from(value: SqlxError) -> Self {
        match value {
            Error::RowNotFound => Self::NotFound,
            _ => Self::DbError
        }
    }
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl ResponseError for ErrorType {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).finish()
    }
}