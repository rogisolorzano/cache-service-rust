use actix_web::error::{JsonPayloadError};
use actix_web::{HttpRequest, HttpResponse, Error};
use crate::common::errors::error_response::{ErrorResponse, ErrorBody};
use actix_web::http::StatusCode;

// Handles a JsonPayloadError. For use as the catchall actix-web json error handler.
pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let error_response = ErrorResponse {
        error: ErrorBody {
            code: StatusCode::BAD_REQUEST.as_u16(),
            message: err.to_string()
        }
    };

    Error::from(
        HttpResponse::BadRequest()
            .json(error_response)
    )
}
