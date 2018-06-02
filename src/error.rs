use http;
#[cfg(feature = "tls")]
use http::uri::{InvalidUri, InvalidUriParts};
use hyper;
use hyper::header::ToStrError;
use serde_json;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "tls")]
    ConvertingLocationFailed(InvalidUriParts),
    DeserializationError(serde_json::Error),
    HTTPHeaderConversionError(ToStrError),
    HTTPError(http::Error),
    HTTPClientError(hyper::Error),
    MissingLocationHeader,
    #[cfg(feature = "tls")]
    ParsingLocationFailed(InvalidUri),
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

#[cfg(feature = "tls")]
impl From<InvalidUriParts> for Error {
    fn from(error: InvalidUriParts) -> Self {
        Error::ConvertingLocationFailed(error)
    }
}

#[cfg(feature = "tls")]
impl From<InvalidUri> for Error {
    fn from(error: InvalidUri) -> Self {
        Error::ParsingLocationFailed(error)
    }
}
