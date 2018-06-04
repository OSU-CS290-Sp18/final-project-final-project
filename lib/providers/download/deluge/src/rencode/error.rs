use bincode;

#[derive(Debug)]
pub enum Error {
    InvalidInteger,
    SerializationError(bincode::Error),
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Error {
        Error::SerializationError(err)
    }
}
