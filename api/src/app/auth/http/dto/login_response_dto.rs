use serde_derive::Serialize;

#[derive(Serialize)]
pub struct LoginResponseDTO {
    pub token: String,
}
