use crate::models::api::*;
use crate::models::rust::*;
use axum::extract::{Path, Query};
use axum::Json;
use serde_json::{json, Value};
use validator::Validate;

pub async fn get(Query(data): Query<RustGetQueryRequest>) -> Result<Json<Value>, ApiError> {
    if data.validate().is_err() {
        return Err(ApiError::ValidationError(data.validate().unwrap_err()));
    }

    Ok(Json(
        json!({"hello": data.name.unwrap_or(String::from("Default name")) }),
    ))
}

pub async fn get_id(Path(id): Path<i32>) -> Json<RustGetIdResponse> {
    Json(RustGetIdResponse { id })
}

pub async fn post(Json(json): Json<RustPostRequest>) -> Json<RustPostResponse> {
    Json(RustPostResponse {
        hello: String::from(json.name),
    })
}
