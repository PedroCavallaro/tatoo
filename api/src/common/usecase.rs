use crate::domain::error::ApiError;

pub trait UseCase<T, I> {
    async fn execute(&self, input: I) -> Result<T, ApiError>;
}
