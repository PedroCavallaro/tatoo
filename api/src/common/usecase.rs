use crate::domain::error::ApiError;

pub trait UseCase<T, I> {
    fn execute(&self, input: I) -> Result<T, ApiError>;
}
