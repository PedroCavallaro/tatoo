#[derive(Clone, Debug)]
pub struct JwtPayload {
    pub id: i64,
    pub name: String,
    pub email: String,
}
