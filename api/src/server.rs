use core::hello_world_service::HelloWorldService;
use std::sync::Arc;

use axum::{middleware, Router};
use domain::configuration::{self, Configuration, KafkaConfiguration};
use infrastructure::db::hello_world_db::HelloWorldDb;

use crate::routes::v0::hello_world::HelloWorldRouter;
use crate::routes::v0::swagger::SwaggerRouter;

use crate::middlewares::response_mapper::ResponseMapper;

/// Start the server on the given ip address
///
/// This function creates all routes required for the backend and starts the server on the given IP address.
///
/// # Panics
/// Panics when given IP address is invalid
pub async fn start_server(ip: &str) {
    let db = Arc::new(HelloWorldDb::new());
    let configuration = Arc::new(Configuration::new(KafkaConfiguration::default()));
    let hello_world_controller = Arc::new(HelloWorldService::new(db, configuration));

    let app = Router::new()
        .merge(HelloWorldRouter::setup_routes(
            hello_world_controller.clone(),
        ))
        .merge(SwaggerRouter::setup_routes())
        .layer(middleware::map_response(
            ResponseMapper::main_response_mapper,
        ));

    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
