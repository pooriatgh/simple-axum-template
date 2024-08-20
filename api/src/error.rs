use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use core::error::CoreError;

#[derive(Debug, Clone)]
pub enum APIError {
    NotFound(String),
    AlreadyExists(String),
    InvalidParams,
    ServiceError(Option<String>),
}

impl APIError {
    pub fn description(&self) -> Option<String> {
        match self {
            Self::NotFound(e) => Some(e.clone()),
            Self::InvalidParams => Some("Invalid parameters".to_string()),
            Self::ServiceError(e) => e.clone(),
            Self::AlreadyExists(e) => Some(e.clone()),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidParams => StatusCode::BAD_REQUEST,
            Self::ServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AlreadyExists(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(_) => write!(f, "NOT_FOUND",),
            Self::InvalidParams => write!(f, "INVALID_PARAMS"),
            Self::ServiceError(_) => write!(f, "SERVICE_ERROR"),
            Self::AlreadyExists(_) => write!(f, "ALREADY_EXISTS"),
        }
    }
}

impl From<CoreError> for APIError {
    fn from(err: CoreError) -> Self {
        match err {
            CoreError::NotFound(e) => APIError::NotFound(e),
            CoreError::AlreadyExists(e) => APIError::AlreadyExists(e),
            CoreError::NotImplemented(e) => APIError::ServiceError(Some(e)),
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        let mut response = self.status_code().into_response();
        response.extensions_mut().insert(self);

        response
    }
}
