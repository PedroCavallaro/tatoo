use std::{error::Error, fmt, io};

#[derive(Debug)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    pub error: Option<Box<dyn Error>>,
}

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> Self {
        ApiError {
            code: 500,
            message: "Database connection error".to_string(),
            error: Some(Box::new(err)),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
