use std::fmt;

use actix::MailboxError;
use actix_web::{HttpResponse, ResponseError};
use postgres;
use r2d2;
use tvmaze;

#[derive(Debug)]
pub enum WebError {
    ActixMailboxError(MailboxError),
    DBError(postgres::Error),
    DBPoolError(r2d2::Error),
    ResourceNotFound(i32),
    TVMazeError(tvmaze::error::Error),
}

impl From<MailboxError> for WebError {
    fn from(err: MailboxError) -> WebError {
        WebError::ActixMailboxError(err)
    }
}

impl From<tvmaze::error::Error> for WebError {
    fn from(err: tvmaze::error::Error) -> WebError {
        WebError::TVMazeError(err)
    }
}

impl From<postgres::Error> for WebError {
    fn from(err: postgres::Error) -> WebError {
        WebError::DBError(err)
    }
}

impl From<r2d2::Error> for WebError {
    fn from(err: r2d2::Error) -> WebError {
        WebError::DBPoolError(err)
    }
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ASDASDAS")
    }
}

impl ::std::error::Error for WebError {
    fn description(&self) -> &str {
        "TODO"
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        None
    }
}

impl ResponseError for WebError {
    fn error_response(&self) -> HttpResponse {
        use self::WebError::*;

        match *self {
            ResourceNotFound(id) => {
                HttpResponse::NotFound().body(format!("Resource not found with id {}", id))
            }
            _ => {
                HttpResponse::InternalServerError().body(format!("Internal Error: {:?}", self))
            }
        }
    }
}

