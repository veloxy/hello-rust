use crate::models::api::*;
use crate::models::rust::*;
use axum::extract::{Path, Query};
use axum::{Extension, Json};
use serde_json::{json, Value};
use validator::Validate;
use sea_orm::*;
use ::entity::{prelude::*, *};
use chrono::prelude::*;

pub async fn get(Query(data): Query<RustGetQueryRequest>) -> Result<Json<Value>, ApiError> {
    if data.validate().is_err() {
        return Err(ApiError::ValidationError(data.validate().unwrap_err()));
    }

    Ok(Json(
        json!({"hello": data.name.unwrap_or(String::from("Default name")) }),
    ))
}

pub async fn get_id(Extension(database): Extension<DatabaseConnection>, Path(id): Path<i32>) -> Result<Json<post::Model>, ApiError> {
    let post: Option<post::Model> = Post::find_by_id(id).one(&database).await?;

    Ok(Json(post.unwrap()))
}

pub async fn post(Extension(database): Extension<DatabaseConnection>, Json(json): Json<RustPostRequest>) -> Result<Json<RustPostResponse>, ApiError> {
    let post = post::ActiveModel {
        title: ActiveValue::Set(json.name),
        text: ActiveValue::Set(String::from("Test")),
        date: ActiveValue::Set(Some(Utc::now().date_naive())),
        ..Default::default()
    };

    let res = Post::insert(post).exec(&database).await?;

    Ok(Json(RustPostResponse {
        id: res.last_insert_id,
    }))
}
