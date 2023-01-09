use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use sea_orm::DbErr;
use serde_json::json;
use validator::ValidationErrors;

pub enum ApiError {
    ValidationError(ValidationErrors),
    DbErr(DbErr),
}

impl From<sea_orm::DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        ApiError::DbErr(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, errors) = match self {
            ApiError::ValidationError(validation_errors) => {
                (StatusCode::BAD_REQUEST, json!({"error": validation_errors}))
            },
            ApiError::DbErr(value) => {
                (StatusCode::INTERNAL_SERVER_ERROR, json!({"error": {"message": value.to_string() }}))
            }
        };

        let body = Json(errors);

        (status, body).into_response()
    }
}
