use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use validator::ValidationErrors;

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
