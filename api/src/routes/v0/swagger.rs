use crate::api_doc::ApiDoc;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct SwaggerRouter;

impl SwaggerRouter {
    pub fn setup_routes() -> Router {
        Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
    }
}
