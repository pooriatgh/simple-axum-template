//! Main component of the backend which ties everything together.
pub mod api_doc;
pub mod error;
pub mod middlewares;
pub mod routes;
pub mod server;
pub use error::APIError;
