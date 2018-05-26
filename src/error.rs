use http;
use hyper;
use hyper::header::ToStrError;
use serde_json;

#[derive(Debug)]
pub enum Error {
    DeserializationError(serde_json::Error),
    HTTPHeaderConversionError(ToStrError),
    HTTPError(http::Error),
    HTTPClientError(hyper::Error),
    MissingLocationHeader,
    ShowNotFound,
    UnexpectedResponse,
}

impl From<http::Error> for Error {
    fn from(error: http::Error) -> Self {
        Error::HTTPError(error)
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Self {
        Error::HTTPClientError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::DeserializationError(error)
    }
}

impl From<ToStrError> for Error {
    fn from(error: ToStrError) -> Self {
        Error::HTTPHeaderConversionError(error)
    }
}
