use serde::{Serialize};

#[derive(Serialize)]
pub struct ErrorResponse {
    pub(crate) error: ErrorBody
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub(crate) code: u16,
    pub(crate) message: String,
}
