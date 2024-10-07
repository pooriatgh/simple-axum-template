use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use core::hello_world_service::HelloWorldService;
use domain::hello_world::HelloWorld;
use std::sync::Arc;

use crate::error::APIError;

#[utoipa::path(
    get,
    path = "/api/hello_world/{key}",
    tag = "hello world",
    responses(
        (status = 200, description = "Get a hello", body = String),
        (status = 404, description = "Not found", body = ClientErrorResponse),
        (status = 500, description = "Internal server error that can be caused by database failure", body = ClientErrorResponse)
    )
)]
pub async fn get_hello_world(
    State(service): State<Arc<HelloWorldService>>,
    Path(key): Path<String>,
) -> Result<Json<HelloWorld>, APIError> {
    let value = service.get_hello_world(key).await?;
    Ok(Json(value))
}

// test health api

#[utoipa::path(
    get,
    path = "/api/health",
    tag = "health",
    responses(
        (status = 200, description = "Get a health", body = String),
        (status = 404, description = "Not found", body = ClientErrorResponse),
        (status = 500, description = "Internal server error that can be caused by database failure", body = ClientErrorResponse)
    )
)]
pub async fn get_health() -> Result<Json<String>, APIError> {
    Ok(Json("OK".to_string()))
}

pub struct HelloWorldRouter;

impl HelloWorldRouter {
    pub fn setup_routes(service: Arc<HelloWorldService>) -> Router {
        Router::new()
            .route("/api/hello_world/:key", get(get_hello_world))
            .route("/api/health", get(get_health))
            .with_state(service)
    }
}
