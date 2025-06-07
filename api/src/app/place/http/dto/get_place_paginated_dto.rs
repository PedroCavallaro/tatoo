pub struct GetPlacePaginatedDTO {
    pub q: String,
    pub page: i32,
    pub limit: i32,
    pub order: String,
}
