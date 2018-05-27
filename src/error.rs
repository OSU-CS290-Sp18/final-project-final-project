use http;
use http::uri::{InvalidUri, InvalidUriParts};
use hyper;
use hyper::header::ToStrError;
#[cfg(feature = "tls")]
use hyper_tls;
use serde_json;

#[derive(Debug)]
pub enum Error {
    ConvertingLocationFailed(InvalidUriParts),
    DeserializationError(serde_json::Error),
    HTTPHeaderConversionError(ToStrError),
    HTTPError(http::Error),
    HTTPClientError(hyper::Error),
    MissingLocationHeader,
    ParsingLocationFailed(InvalidUri),
    ShowNotFound,
    #[cfg(feature = "tls")]
    TLSError(hyper_tls::Error),
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
impl From<hyper_tls::Error> for Error {
    fn from(error: hyper_tls::Error) -> Self {
        Error::TLSError(error)
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
