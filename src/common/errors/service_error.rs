use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use thiserror::Error;
use crate::common::errors::error_response::{ErrorResponse, ErrorBody};

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Invalid field: {field}. Expected: {expected}.")]
    Validation { field: String, expected: String},
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::Validation { .. } => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            error: ErrorBody {
                code: status_code.as_u16(),
                message: self.to_string(),
            }
        };

        HttpResponse::build(status_code).json(error_response)
    }
}
