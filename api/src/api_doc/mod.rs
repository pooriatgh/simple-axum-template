use domain::hello_world::HelloWorld;
use utoipa::OpenApi;

use crate::middlewares::response_mapper::ClientErrorResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::v0::hello_world::get_hello_world,
        crate::routes::v0::hello_world::get_health
    ),
    components(schemas(HelloWorld, ClientErrorResponse))
)]
pub struct ApiDoc;
