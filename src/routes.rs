pub mod hello;

use axum::{routing::get, Router, Extension};
use sea_orm::DatabaseConnection;

pub fn create_routes(connection: DatabaseConnection) -> Router {
    Router::new()
        .route("/hello/rust", get(hello::rust::get).post(hello::rust::post))
        .route("/hello/rust/:id", get(hello::rust::get_id))
        .layer(Extension(connection))
}
