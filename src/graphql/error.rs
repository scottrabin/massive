use hyper::Method;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum GQLError {
    GenericError(String),
    HyperError(hyper::Error),
    SerializationError(serde_json::error::Error),
    UnrecognizedMethod(Method),
}

impl std::error::Error for GQLError {}

impl std::fmt::Display for GQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GQLError::UnrecognizedMethod(method) => {
                write!(f, "Unrecognized HTTP method: {}", method)
            }
            GQLError::GenericError(cause) => write!(f, "Response builder failed: {}", cause),
            GQLError::SerializationError(err) => write!(f, "Unable to serialize response: {}", err),
            GQLError::HyperError(err) => write!(f, "Error in hyper: {}", err),
        }
    }
}

impl From<hyper::http::Error> for GQLError {
    fn from(err: hyper::http::Error) -> Self {
        GQLError::GenericError(format!("Response builder failed: {}", err))
    }
}

impl From<serde_json::error::Error> for GQLError {
    fn from(err: serde_json::error::Error) -> Self {
        GQLError::SerializationError(err)
    }
}

impl From<hyper::Error> for GQLError {
    fn from(err: hyper::Error) -> Self {
        GQLError::HyperError(err)
    }
}

impl From<FromUtf8Error> for GQLError {
    fn from(err: FromUtf8Error) -> Self {
        GQLError::GenericError(err.to_string())
    }
}
