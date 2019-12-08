use crate::{error::Error, Result};
use log::error;
use tide::{IntoResponse, Response};

static INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

pub struct HandlerResponse {
    response: Result<Response>,
}

impl HandlerResponse {
    pub fn new(response: Result<Response>) -> Self {
        HandlerResponse { response }
    }
}

impl IntoResponse for HandlerResponse {
    fn into_response(self) -> Response {
        match self.response {
            Ok(r) => r,
            Err(e) => e.into_response(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::ParseJSONBodyError(_) => Response::new(400).body_string(self.to_string()),
            _ => {
                error!("unhandler error: {}", self);
                Response::new(500).body_string(INTERNAL_SERVER_ERROR.to_string())
            }
        }
    }
}
