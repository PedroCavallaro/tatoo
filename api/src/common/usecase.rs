use crate::domain::error::ApiError;

pub trait UseCase<T, I> {
    fn execute(input: I) -> Result<T, ApiError>;
}
