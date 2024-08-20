//! Response Mapper
//!
//! This module contains the response mapper that maps the response to a new
//! client error response if an error occured. This makes sure that the client
//! gets a proper error response and that no technical/secrets are given to the
//! client that may be in the original error.

// TODO: Implement storing logging of the request and response.

use axum::{
    http::{Method, Uri},
    response::{IntoResponse, Response},
    Json,
};

use log::{error, info};
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;

use crate::error::APIError;

pub struct ResponseMapper;

#[derive(Serialize, ToSchema)]
pub struct ClientErrorResponse {
    type_: String,
    req_uuid: String,
    description: Option<String>,
}

impl ResponseMapper {
    /// Main response mapper
    /// Maps the response to a new client error response if there is an error and can log it (not implemented yet).
    ///
    /// Based on https://github.com/jeremychone-channel/rust-axum-course/blob/main/src/main.rs
    pub async fn main_response_mapper(uri: Uri, req_method: Method, res: Response) -> Response {
        let uuid = uuid::Uuid::new_v4();

        // -- Get the eventual response error.
        let service_error = res.extensions().get::<APIError>();

        let mut web_error = None;
        // -- If client error, build the new reponse.
        let error_response = service_error.map(|error| {
            let client_error_body = ClientErrorResponse {
                type_: format!("{}", error),
                req_uuid: uuid.to_string(),
                description: error.description(),
            };

            let client_error_body = json!(client_error_body);

            error!("    ->> client_error_body: {client_error_body}");
            web_error = Some(error);
            // Build the new response from the client_error_body
            (error.status_code(), Json(client_error_body)).into_response()
        });

        // Todo: Log the request and response on a proper place.
        Self::log_request_response(uuid, req_method, uri, &res, web_error);

        error_response.unwrap_or(res)
    }

    fn log_request_response(
        uuid: uuid::Uuid,
        req_method: Method,
        uri: Uri,
        res: &Response,
        error: Option<&APIError>,
    ) {
        info!("    ->> log_request:\t{uuid}, {req_method}, {uri}");

        if let Some(error) = error {
            error!("    ->> log_response:\t{uuid}, {res:?}, {error:?}");
        } else {
            info!("    ->> log_response:\t{uuid}, {res:?}, {error:?}");
        }
        info!("");
    }
}
