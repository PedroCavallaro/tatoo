use std::{fmt, io};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_derive::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}

impl From<Box<dyn std::error::Error>> for ApiError {
    fn from(_: Box<dyn std::error::Error>) -> Self {
        ApiError {
            code: 500,
            message: "An error occurred".into(),
        }
    }
}

impl ApiError {
    pub fn new(code: u16, message: &str) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        println!("Err");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl From<io::Error> for ApiError {
    fn from(_: io::Error) -> Self {
        ApiError {
            code: 500,
            message: "An error ocurred".into(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
