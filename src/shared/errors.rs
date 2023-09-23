use thiserror::Error as ThisError;
use sqlx::Error as SqlxError;
pub type Result<T> = std::result::Result<T, CustomError>;

#[derive(Debug, Clone, ThisError)]
pub enum CustomError{
    #[error("An error occurred during database interaction. {0}")]
    DatabaseError(String)
}

impl From<SqlxError> for CustomError {
    fn from(sqlx_error: SqlxError) -> Self {
        match sqlx_error.as_database_error() {
            Some(db_error) => CustomError::DatabaseError(db_error.to_string()),
            None => {
                eprintln!("error {:?}", sqlx_error);
                CustomError::DatabaseError(String::from("Unrecognized database error!"))
            }
        }
    }
}