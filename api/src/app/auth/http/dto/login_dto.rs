use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginDTO {
    pub email: String,
}
