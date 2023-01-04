use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use validator::{Validate, ValidationErrors};

pub enum ApiError {
    ValidationError(ValidationErrors),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, errors) = match self {
            ApiError::ValidationError(validation_errors) => {
                (StatusCode::BAD_REQUEST, validation_errors)
            }
        };

        let body = Json(json!({
            "error": errors,
        }));

        (status, body).into_response()
    }
}

#[derive(Deserialize, Validate)]
pub struct RustGetQueryRequest {
    #[validate(length(min = 5))]
    name: Option<String>,
}

pub async fn get(Query(data): Query<RustGetQueryRequest>) -> Result<Json<Value>, ApiError>{
    if data.validate().is_err() {
        return Err(ApiError::ValidationError(data.validate().unwrap_err()));
    }

    Ok(Json(json!({"hello": data.name.unwrap_or(String::from("Default name")) })))
}

#[derive(Serialize)]
pub struct RustGetIdResponse {
    id: i32,
}

pub async fn get_id(Path(id): Path<i32>) -> Json<RustGetIdResponse> {
    Json(RustGetIdResponse { id })
}

#[derive(Deserialize)]
pub struct RustPostRequest {
    name: String,
}

#[derive(Serialize)]
pub struct RustPostResponse {
    hello: String,
}

pub async fn post(Json(json): Json<RustPostRequest>) -> Json<RustPostResponse> {
    Json(RustPostResponse {hello: String::from(json.name)})
}