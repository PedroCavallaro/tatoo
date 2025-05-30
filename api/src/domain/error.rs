use std::{fmt, io};

#[derive(Debug)]
pub struct ApiError<'a> {
    pub code: u16,
    pub message: &'a str,
}

impl<'a> ApiError<'a> {
    pub fn new(code: u16, message: &'a str) -> Self {
        Self { code, message }
    }
}

impl<'a> From<io::Error> for ApiError<'a> {
    fn from(_: io::Error) -> Self {
        ApiError {
            code: 500,
            message: "An error ocurred",
        }
    }
}

impl<'a> fmt::Display for ApiError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
