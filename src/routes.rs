pub mod hello;

use axum::{Router, routing::get};

pub fn create_routes() -> Router {
    Router::new()
        .route("/hello/rust", get(hello::rust::get).post(hello::rust::post))
        .route("/hello/rust/:id", get(hello::rust::get_id))
}