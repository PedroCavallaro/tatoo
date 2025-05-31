use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub sub: String,
    pub name: String,
    pub picture: String,
}
