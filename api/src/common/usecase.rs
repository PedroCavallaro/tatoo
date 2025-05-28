use std::error::Error;

pub trait UseCase<I, T> {
    fn execute(input: I) -> Result<T, Box<dyn Error>>;
}
