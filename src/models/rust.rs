use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RustGetQueryRequest {
    #[validate(length(min = 5))]
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct RustPostRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct RustPostResponse {
    pub id: i32,
}

#[derive(Serialize)]
pub struct RustGetIdResponse {
    pub id: i32,
}
