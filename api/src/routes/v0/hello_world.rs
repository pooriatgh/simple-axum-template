use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use core::hello_world_service::HelloWorldService;
use domain::hello_world::HelloWorld;

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
    State(service): State<HelloWorldService>,
    Path(key): Path<String>,
) -> Result<Json<HelloWorld>, APIError> {
    let value = service.get_hello_world(key).await?;
    Ok(Json(value))
}

pub struct HelloWorldRouter;

impl HelloWorldRouter {
    pub fn setup_routes(service: HelloWorldService) -> Router {
        Router::new()
            .route("/api/hello_world", get(get_hello_world))
            .with_state(service)
    }
}
