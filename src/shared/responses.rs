use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse {
    message: String,
    date_time: DateTime<Local>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
    info: String,
    date_time: DateTime<Local>,
}

impl MessageResponse {
    pub fn new(message: String) -> Self {
        Self {
            message,
            date_time: Local::now(),
        }
    }
}